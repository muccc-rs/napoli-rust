use napoli_lib::napoli::order_service_server::OrderService;
use napoli_lib::napoli::{
    AddOrderEntryRequest, CreateOrderRequest, GetOrdersReply, GetOrdersRequest, OrderEntryRequest,
    SetOrderEntryPaidRequest, SingleOrderReply,
};

use napoli_server_persistent_entities::order;
use napoli_server_persistent_entities::order_entry;
use sea_orm::{ActiveModelTrait, ModelTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{IntoActiveModel, QueryOrder as _};
use tonic::{Request, Response, Status};

use crate::errors::map_to_status;
use crate::model_adapters::{self, get_order_entry_from_add_request};

pub struct NapoliServer {
    db_handle: DatabaseConnection,
}

#[tonic::async_trait]
impl OrderService for NapoliServer {
    async fn get_orders(
        &self,
        request: Request<GetOrdersRequest>,
    ) -> Result<Response<GetOrdersReply>, Status> {
        println!("Got a request: {:?}", request);

        let orders = order::Entity::find()
            .order_by_desc(order::Column::Id)
            .find_with_related(order_entry::Entity)
            .all(&self.db_handle)
            .await
            .map_err(map_to_status)?;

        let orders = orders
            .into_iter()
            .map(|(order, entries)| {
                model_adapters::database_order_to_tonic_order(order, entries.into_iter())
            })
            .collect();

        Ok(Response::new(GetOrdersReply { orders }))
    }

    async fn create_order(
        &self,
        request: tonic::Request<CreateOrderRequest>,
    ) -> Result<Response<SingleOrderReply>, Status> {
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
        request: tonic::Request<AddOrderEntryRequest>,
    ) -> Result<Response<SingleOrderReply>, Status> {
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
                Ok(Response::new(order))
            }
        }
    }

    async fn remove_order_entry(
        &self,
        request: Request<OrderEntryRequest>,
    ) -> Result<Response<SingleOrderReply>, Status> {
        todo!()
    }

    async fn set_order_entry_paid(
        &self,
        request: Request<SetOrderEntryPaidRequest>,
    ) -> Result<Response<SingleOrderReply>, Status> {
        todo!()
    }
}

impl NapoliServer {
    pub fn with_connection(db_handle: DatabaseConnection) -> Self {
        NapoliServer { db_handle }
    }
}
