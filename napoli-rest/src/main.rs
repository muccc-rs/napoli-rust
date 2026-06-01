use clap::Parser;
use napoli_server_migrations::{Migrator, MigratorTrait};
use napoli_rest::routes::AppState;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    bind_addr: String,
    #[clap(short, long, default_value = "napoli.sqlite")]
    sqlite_file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !std::path::Path::new(&args.sqlite_file_name).exists() {
        println!("Creating database file: {}", args.sqlite_file_name);
        std::fs::File::create(&args.sqlite_file_name)?;
    }

    let db = sea_orm::Database::connect(format!("sqlite://{}", &args.sqlite_file_name)).await?;
    Migrator::up(&db, None).await?;

    let app = napoli_rest::build_app(AppState { db });

    println!("napoli-rest listening on {}", args.bind_addr);
    let listener = tokio::net::TcpListener::bind(&args.bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
