use deadpool_redis::{Config, Runtime};
use tracing::info;

pub async fn init_pool() -> deadpool_redis::Pool {
    info!("Initializing the Redis connection pool");
    let url = dotenvy::var("REDIS_URL").expect(".env file does not contain 'REDIS_URL'");

    Config::from_url(url)
        .builder()
        .expect("Error building Redis pool")
        .max_size(10000)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Redis connection failed")
}
