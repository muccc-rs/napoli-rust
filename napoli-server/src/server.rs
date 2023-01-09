use napoli_lib::napoli::order_service_server::{OrderService, OrderServiceServer};
use napoli_lib::napoli::{GetOrdersReply, GetOrdersRequest, FILE_DESCRIPTOR_SET};
use napoli_server_migrations::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tonic::{transport::Server, Response, Status};

const DATABASE_FILE_NAME: &str = "napoli.sqlite";

// #[derive(Default)]
pub struct NapoliServer {
    db_handle: DatabaseConnection,
}

#[tonic::async_trait]
impl OrderService for NapoliServer {
    async fn get_orders(
        &self,
        request: tonic::Request<GetOrdersRequest>,
    ) -> Result<Response<GetOrdersReply>, Status> {
        println!("Got a request: {:?}", request);

        let orders = napoli_server_persistent_entities::order::Entity::find()
            .all(&self.db_handle)
            .await
            .unwrap();

        Ok(Response::new(GetOrdersReply {
            orders: orders
                .into_iter()
                .map(|po| napoli_lib::napoli::Order {
                    id: format!("i{}", po.id),
                    menu_url: "lmao".to_string(),
                    state: napoli_lib::napoli::OrderState::Open.into(),
                    entries: vec![],
                })
                .collect(),
        }))
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
