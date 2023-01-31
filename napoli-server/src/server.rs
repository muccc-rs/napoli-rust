mod model_adapters;
use std::fmt::Display;

use napoli_lib::napoli::order_service_server::{OrderService, OrderServiceServer};
use napoli_lib::napoli::{
    AddOrderEntryRequest, CreateOrderRequest, GetOrdersReply, GetOrdersRequest, OrderEntryRequest,
    SingleOrderReply, FILE_DESCRIPTOR_SET, SetOrderEntryPaidRequest,
};
use napoli_server_migrations::{Migrator, MigratorTrait};
use napoli_server_persistent_entities::order_entry;
use sea_orm::{ActiveModelTrait, ModelTrait, Set};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tonic::{transport::Server, Request, Response, Status};

const DATABASE_FILE_NAME: &str = "napoli.sqlite";

// fn map_err_to_status(err: anyhow::Error) -> Status {
//     Status::internal(err.to_string())
// }

// fn map_err_to_status(err: sea_orm::DbErr) -> Status {
//     Status::internal(err.to_string())
// }

fn map_err_to_status<T>(err: T) -> Status where T: Display {
    Status::internal(err.to_string())
}

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

        // Get all Orders from the database
        let orders = napoli_server_persistent_entities::order::Entity::find()
            .all(&self.db_handle)
            .await;

        // Convert to our protobuf type
        match orders {
            Ok(orders) => Ok(Response::new(GetOrdersReply {
                orders: futures::future::join_all(orders.into_iter().map(|order| async {
                    let order_entries = order
                        .find_related(order_entry::Entity)
                        .all(&self.db_handle)
                        .await
                        .unwrap();
                    model_adapters::make_single_order_reply(order, order_entries)
                        .order
                        .unwrap()
                }))
                .await,
            })),
            Err(error) => {
                let error_msg = format!("Error getting orders: {:?}", error);
                println!("{}", error_msg);
                Err(Status::internal(error_msg))
            }
        }
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

        let order = napoli_server_persistent_entities::order::Entity::find_by_id(request.order_id)
            .one(&self.db_handle)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        match order {
            Some(order) => {
                if order.order_state != napoli_lib::napoli::OrderState::Open as i32 {
                    return Err(Status::invalid_argument("Order is not open"));
                }
            }
            None => return Err(Status::invalid_argument("Order does not exist")),
        }

        // Add order entry
        let order_entry = order_entry::ActiveModel {
            order_id: Set(request.order_id),
            buyer: Set(request.buyer),
            food: Set(request.food),
            paid: Set(false),
            ..Default::default()
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

fn grpc_check_err<T>(res: anyhow::Result<T>) -> std::result::Result<T, Status> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

impl NapoliServer {
    pub fn with_connection(db_handle: DatabaseConnection) -> Self {
        NapoliServer { db_handle }
    }
}

fn assert_file_exists(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(file_name).exists() {
        println!("Database file already exists; skipping creating");
    } else {
        println!(
            "Database file does not exists; creating {}",
            DATABASE_FILE_NAME
        );
        std::fs::File::create(file_name)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_file_exists(DATABASE_FILE_NAME)?;
    let conn = format!("sqlite://{}", DATABASE_FILE_NAME);
    let db = Database::connect(conn).await?;

    Migrator::up(&db, None).await?;

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = NapoliServer::with_connection(db);

    println!("NapoliServer listening on {}", addr);

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(OrderServiceServer::new(greeter))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}
