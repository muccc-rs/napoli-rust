mod errors;
mod model_adapters;
mod server;

use napoli_lib::napoli::order_service_server::OrderServiceServer;
use napoli_lib::napoli::FILE_DESCRIPTOR_SET;
use napoli_server_migrations::{Migrator, MigratorTrait};

use crate::server::NapoliServer;

const DATABASE_FILE_NAME: &str = "napoli.sqlite";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_db_file_exists(DATABASE_FILE_NAME)?;
    let conn = format!("sqlite://{}", DATABASE_FILE_NAME);
    let db = sea_orm::Database::connect(conn).await?;

    Migrator::up(&db, None).await?;

    let addr = match "[::1]:50051".parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Failed to parse address: {}", e);
            return Ok(());
        }
    };

    println!("NapoliServer listening on {}", addr);
    let napoli_server = NapoliServer::with_connection(db);
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    tonic::transport::Server::builder()
        .add_service(OrderServiceServer::new(napoli_server))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}

fn assert_db_file_exists(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(file_name).exists() {
        println!("Database file already exists; skipping creating");
    } else {
        println!(
            "Database file does not exists; creating {}",
            file_name
        );
        std::fs::File::create(file_name)?;
    }
    Ok(())
}
