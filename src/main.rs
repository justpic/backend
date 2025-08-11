mod database;
mod error;
mod models;
mod routes;
mod services;

use std::sync::Arc;

use database::{Repositories, redis::Redis};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Repositories>,
    pub redis: Arc<Redis>,
}

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

    let db = Arc::new(Repositories::new(&db_pool));
    let redis = Arc::new(Redis::new().await);
    let state = Arc::new(AppState { db, redis });

    info!("Running server...");
    let server_host = dotenvy::var("HOST_URL").expect(".env does not contain server host url");

    let app = routes::config().with_state(state);
    let listener = TcpListener::bind(&server_host).await?;

    info!("Server listened on [{server_host}]");
    axum::serve(listener, app).await?;
    Ok(())
}
