mod errors;
mod model_adapters;
mod server;

use napoli_lib::napoli::order_service_server::OrderServiceServer;
use napoli_lib::napoli::FILE_DESCRIPTOR_SET;
use napoli_server_migrations::{Migrator, MigratorTrait};
use tonic_web::GrpcWebLayer;
use tower_http::cors;

use crate::server::NapoliServer;

use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long, default_value = "[::1]:50051")]
    bind_addr: String,
    #[clap(short, long, default_value = "napoli.sqlite")]
    sqlite_file_name: String,
    #[clap(short, long, default_value = "60")]
    gc_delay: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    assert_db_file_exists(&args.sqlite_file_name)?;
    let conn = format!("sqlite://{}", &args.sqlite_file_name);
    let db = sea_orm::Database::connect(conn).await?;

    Migrator::up(&db, None).await?;

    let addr = match args.bind_addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Failed to parse address: {}", e);
            return Ok(());
        }
    };

    println!("NapoliServer listening on {}", addr);
    let napoli_server = NapoliServer::with_connection(db);

    let order_service_server = OrderServiceServer::new(napoli_server);
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let cors = cors::CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([http::Method::POST])
        .allow_origin(cors::Any);

    tonic::transport::Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(order_service_server)
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}

fn assert_db_file_exists(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(file_name).exists() {
        println!("Database file already exists; skipping creating");
    } else {
        println!("Database file does not exists; creating {}", file_name);
        std::fs::File::create(file_name)?;
    }
    Ok(())
}
