use futures::lock::Mutex;
use futures::Stream;
use napoli_lib::napoli as npb;
use std::collections;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use napoli_server_persistent_entities::order;
use napoli_server_persistent_entities::order_entry;
use sea_orm::{ActiveModelTrait, ModelTrait, QueryTrait, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{IntoActiveModel, QueryOrder as _};
use tonic::{Request, Response, Status};

use crate::errors::map_to_status;
use crate::model_adapters::{self, get_order_entry_from_add_request};

pub struct NapoliServer {
    db_handle: DatabaseConnection,
    pub active_order_update_streams: Arc<Mutex<collections::BTreeMap<i32, Vec<OrderSender>>>>,
}

type GRPCResultResponse<T> = Result<Response<T>, Status>;
type GRPCReplyResult<T> = Result<T, Status>;
type OrderSender = mpsc::Sender<GRPCReplyResult<npb::SingleOrderReply>>;
type OrderUpdateStream = Pin<Box<dyn Stream<Item = GRPCReplyResult<npb::SingleOrderReply>> + Send>>;

#[tonic::async_trait]
impl npb::order_service_server::OrderService for NapoliServer {
    type StreamOrderUpdatesStream = OrderUpdateStream;

    async fn stream_order_updates(
        &self,
        req: tonic::Request<npb::GetOrderRequest>,
    ) -> GRPCResultResponse<Self::StreamOrderUpdatesStream> {
        /*
           A stream is a basically a future that is fulfilled several times.
           This function is supposed to return a stream that represents the updates of the requested order.

           We create a new channel for each stream request that is stored in a hashmap
           and can be accessed by the order id (thus pushed updates to).
        */
        println!("stream_order_updates: Got a request: {:?}", req);
        let order_id = req.into_inner().order_id;

        let (tx, rx) = mpsc::channel(128);
        self.active_order_update_streams
            .lock()
            .await
            .entry(order_id)
            .or_default()
            .push(tx);

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamOrderUpdatesStream
        ))
    }

    async fn get_orders(
        &self,
        request: Request<npb::GetOrdersRequest>,
    ) -> Result<Response<npb::GetOrdersReply>, Status> {
        println!("Got a request: {:?}", request);

        let orders_query = order::Entity::find()
            .order_by(order::Column::Id, sea_orm::Order::Desc)
            .find_with_related(order_entry::Entity);

        println!(
            "Query: {:?}",
            orders_query
                .build(sea_orm::DatabaseBackend::Sqlite)
                .to_string()
        );

        let orders = orders_query
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let orders = orders
            .into_iter()
            .map(|(order, entries)| {
                println!("Order: {:?}", order);
                // println!("Entries: {:?}", entries);
                model_adapters::database_order_to_tonic_order(order, entries.into_iter())
            })
            .collect();

        Ok(Response::new(npb::GetOrdersReply { orders }))
    }

    async fn get_order(
        &self,
        request: Request<npb::GetOrderRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let order_id = request.into_inner().order_id;
        let orders = order::Entity::find_by_id(order_id)
            .find_with_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let (order, entries) = match orders.into_iter().next() {
            Some((order, entries)) => (order, entries),
            None => return Err(Status::not_found("order not found")),
        };

        Ok(Response::new(npb::SingleOrderReply {
            order: Some(model_adapters::database_order_to_tonic_order(
                order,
                entries.into_iter(),
            )),
        }))
    }

    async fn create_order(
        &self,
        request: tonic::Request<npb::CreateOrderRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let order = match model_adapters::get_order_from_create_request(request.into_inner()) {
            Some(order) => order,
            None => return Err(Status::internal("no order non")),
        };
        println!("New Order: {:?}", order);
        let order = order
            .insert(&self.db_handle)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        // TODO(q3k): error handling
        let order_entries = order
            .find_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        let ok_order = model_adapters::make_single_order_reply(order, order_entries);
        Ok(Response::new(ok_order))
    }

    async fn add_order_entry(
        &self,
        request: tonic::Request<npb::AddOrderEntryRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let request = request.into_inner();

        let order = napoli_server_persistent_entities::order::Entity::find_by_id(
            request.order_id.to_owned(),
        )
        .one(&self.db_handle)
        .await
        .map_err(|err| Status::internal(err.to_string()))?;
        match order {
            Some(order) => {
                if order.state != napoli_lib::napoli::OrderState::Open as i32 {
                    return Err(Status::invalid_argument("Order is not open"));
                }
            }
            None => return Err(Status::invalid_argument("Order does not exist")),
        }

        // Add order entry
        let order_entry = get_order_entry_from_add_request(request.to_owned());
        let order_entry = match order_entry {
            Some(order_entry) => order_entry,
            None => return Err(Status::internal("Order entry parse error")),
        };

        order_entry
            .insert(&self.db_handle)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        // Return the order
        let order = napoli_server_persistent_entities::order::Entity::find_by_id(request.order_id)
            .one(&self.db_handle)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        match order {
            None => Err(Status::internal("Order disappeared")),
            Some(order) => {
                let order_entries = order
                    .find_related(order_entry::Entity)
                    .all(&self.db_handle)
                    .await
                    .map_err(|err| Status::internal(err.to_string()))?;
                let order = model_adapters::make_single_order_reply(order, order_entries);

                self.notify_order_changed(&order.order.as_ref().unwrap())
                    .await;
                Ok(Response::new(order))
            }
        }
    }

    async fn update_order_state(
        &self,
        request: Request<npb::UpdateOrderStateRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let request = request.into_inner();
        let order_id = request.order_id.to_owned();
        let orders = order::Entity::find_by_id(order_id)
            .one(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let order = match orders {
            Some(order) => order,
            None => return Err(Status::not_found("order not found")),
        };

        let mut order = order.into_active_model();
        order.state = Set(request.state.to_owned());

        let order: order::Model = order.update(&self.db_handle).await.map_err(map_to_status)?;
        let entries = order
            .find_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let order = model_adapters::database_order_to_tonic_order(order, entries.into_iter());
        self.notify_order_changed(&order).await;

        Ok(Response::new(npb::SingleOrderReply { order: Some(order) }))
    }

    async fn remove_order_entry(
        &self,
        request: Request<npb::OrderEntryRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let request = request.into_inner();

        let _order_entry = order_entry::Entity::delete_by_id(request.order_entry_id)
            .exec(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let orders = order::Entity::find_by_id(request.order_id)
            .find_with_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let (order, entries) = match orders.into_iter().next() {
            Some(order) => order,
            None => return Err(Status::not_found("order not found")),
        };

        let order = model_adapters::database_order_to_tonic_order(order, entries.into_iter());
        self.notify_order_changed(&order).await;

        Ok(Response::new(npb::SingleOrderReply { order: Some(order) }))
    }

    async fn set_order_entry_paid(
        &self,
        request: Request<npb::SetOrderEntryPaidRequest>,
    ) -> Result<Response<npb::SingleOrderReply>, Status> {
        let request = request.into_inner();

        let order_entry = order_entry::Entity::find_by_id(request.order_entry_id)
            .one(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let order_entry = match order_entry {
            Some(order_entry) => order_entry,
            None => return Err(Status::not_found("order entry not found")),
        };

        let mut order_entry = order_entry.into_active_model();
        order_entry.paid = Set(request.paid);

        let _order_entry: order_entry::Model = order_entry
            .update(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let orders = order::Entity::find_by_id(request.order_id)
            .find_with_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let (order, entries) = match orders.into_iter().next() {
            Some(order) => order,
            None => return Err(Status::not_found("order not found")),
        };

        let order = model_adapters::database_order_to_tonic_order(order, entries.into_iter());
        self.notify_order_changed(&order).await;

        Ok(Response::new(npb::SingleOrderReply { order: Some(order) }))
    }
}

impl NapoliServer {
    pub fn with_connection(db_handle: DatabaseConnection) -> Self {
        NapoliServer {
            db_handle,
            active_order_update_streams: Default::default(),
        }
    }

    async fn notify_order_changed(&self, order: &napoli_lib::napoli::Order) {
        let mut streams = self.active_order_update_streams.lock().await;
        if let Some(streams) = streams.get_mut(&order.id) {
            for stream in streams.iter_mut() {
                stream
                    .send(Ok(npb::SingleOrderReply {
                        order: Some(order.clone()),
                    }))
                    .await
                    .ok();
            }
        }
    }
}

pub async fn garbage_collect_update_streams(
    order_update_streams: &Mutex<collections::BTreeMap<i32, Vec<OrderSender>>>,
) {
    println!("Garbage collecting napoli_server");
    let mut streams = order_update_streams.lock().await;
    for e in streams.values_mut() {
        e.retain(|e| {
            let should_retain = !e.is_closed();
            if !should_retain {
                println!("Removed item: {:?}", e);
            }
            should_retain
        });
    }
    streams.retain(|_k, v| !v.is_empty());
}
