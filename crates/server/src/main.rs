use actix_web::{App, HttpServer, web};
use justpic_database;
use justpic_result::Error;
use tracing::info;

mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub redis_pool: deadpool_redis::Pool,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool, redis_pool: deadpool_redis::Pool) -> Self {
        info!("AppState initialized");
        AppState { pool, redis_pool }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .init();
    info!("Tracer initialized");

    dotenvy::dotenv().ok();

    let pool = justpic_database::postgres::init_pool()
        .await
        .expect("Database connection failed");

    justpic_database::postgres::apply_migrations()
        .await
        .expect("An error occurred while running migrations");

    let redis = justpic_database::redis::init_pool().await;

    let state = AppState::new(pool, redis);

    info!("Running justpic server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.pool.clone()))
            .app_data(web::Data::new(state.redis_pool.clone()))
            .configure(routes::config)
    })
    .bind(dotenvy::var("HOST_URL").unwrap())?
    .run()
    .await
}
