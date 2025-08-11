mod error;
mod database;
mod models;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .init();

    let db_pool = database::postgres::init_pool().await?;
    database::postgres::run_migrations().await?;

    info!("Running server...");
    let server_host = dotenvy::var("HOST_URL")
        .expect(".env does not contain server host url");

    let app = Router::new();
    let listener = TcpListener::bind(&server_host).await?;

    info!("Server listened on [{server_host}]");
    axum::serve(listener, app).await?;
    Ok(())
}
