use actix_web::web;
use sqlx::Postgres;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<Postgres>,
    pub redis_pool: deadpool_redis::Pool,
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .init();
}

pub fn setup(pool: sqlx::Pool<Postgres>, redis_pool: deadpool_redis::Pool) -> AppState {
    info!("Running justpic-server");

    AppState { pool, redis_pool }
}

pub fn config(cfg: &mut web::ServiceConfig, state: AppState) {
    cfg.app_data(web::Data::new(state));
}
