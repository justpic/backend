use sqlx::Postgres;
use tracing::info;

pub struct AppConfig {
    pub pool: sqlx::Pool<Postgres>,
    pub redis_pool: deadpool_redis::Pool,
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .init();
}

pub fn setup(pool: sqlx::Pool<Postgres>, redis_pool: deadpool_redis::Pool) -> AppConfig {
    init_tracing();

    info!("Running justpic-server");

    AppConfig { pool, redis_pool }
}
