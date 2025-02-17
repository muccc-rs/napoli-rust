use futures::lock::Mutex;
use futures::Stream;
use napoli_lib::napoli as npb;
use std::collections;
use std::pin::Pin;
use std::sync::Arc;

use napoli_server_persistent_entities::order;
use napoli_server_persistent_entities::order_entry;
use sea_orm::{ActiveModelTrait, ModelTrait, QueryTrait, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{IntoActiveModel, QueryOrder as _};
use tonic::{Request, Response, Status};

use crate::errors::map_to_status;
use crate::model_adapters::{self, get_order_entry_from_add_request};
use crate::validate;

pub struct NapoliServer {
    db_handle: DatabaseConnection,
    pub active_order_update_senders: Arc<Mutex<collections::BTreeMap<i32, OrderSender>>>,
}

type OrderSender = tokio::sync::watch::Sender<tonic::Result<npb::SingleOrderReply>>;

#[tonic::async_trait]
impl npb::order_service_server::OrderService for NapoliServer {
    type StreamOrderUpdatesStream =
        Pin<Box<dyn Stream<Item = tonic::Result<npb::SingleOrderReply>> + Send>>;

    async fn stream_order_updates(
        &self,
        req: tonic::Request<npb::GetOrderRequest>,
    ) -> tonic::Result<tonic::Response<Self::StreamOrderUpdatesStream>> {
        println!("stream_order_updates: Got a request: {:?}", req);
        let order_id = req.into_inner().order_id;

        let initial_order = self
            .get_order(tonic::Request::new(npb::GetOrderRequest { order_id }))
            .await?
            .into_inner();

        let mut senders = self.active_order_update_senders.lock().await;
        let rx = match senders.entry(order_id) {
            collections::btree_map::Entry::Occupied(entry) => entry.get().subscribe(),
            collections::btree_map::Entry::Vacant(entry) => {
                let (tx, rx) = tokio::sync::watch::channel(Ok(initial_order));
                entry.insert(tx);
                rx
            }
        };

        let output_stream = tokio_stream::wrappers::WatchStream::new(rx);

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
        let request = request.into_inner();

        validate::length("menu_url", &request.menu_url)?;

        let order = match model_adapters::get_order_from_create_request(request) {
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

        validate::length("food", &request.food)?;
        validate::length("buyer", &request.buyer)?;

        let order_entry = match order_entry {
            Some(order_entry) => order_entry,
            None => return Err(Status::internal("Order entry parse error")),
        };

        // lmao this api
        if *order_entry.price_in_millicents.as_ref() > 10_000_00_000 {
            return Err(tonic::Status::invalid_argument("bro that's way too expensive bro"));
        }

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

                self.notify_order_changed(order.order.as_ref().unwrap())
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
            active_order_update_senders: Default::default(),
        }
    }

    async fn notify_order_changed(&self, order: &napoli_lib::napoli::Order) {
        let mut senders = self.active_order_update_senders.lock().await;
        if let Some(sender) = senders.get_mut(&order.id) {
            sender
                .send(Ok(npb::SingleOrderReply {
                    order: Some(order.clone()),
                }))
                .ok();
        }
    }
}
