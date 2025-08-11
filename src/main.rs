mod database;
mod error;
mod models;
mod routes;

use std::sync::Arc;

use database::repositories::Repositories;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

pub struct AppState {
    repos: Arc<Repositories>,
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

    let repos = Arc::new(Repositories::new(&db_pool));
    let state = Arc::new(AppState { repos });

    info!("Running server...");
    let server_host = dotenvy::var("HOST_URL").expect(".env does not contain server host url");

    let app = routes::config().with_state(state);
    let listener = TcpListener::bind(&server_host).await?;

    info!("Server listened on [{server_host}]");
    axum::serve(listener, app).await?;
    Ok(())
}
