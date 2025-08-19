use deadpool_redis::{Config, Runtime};
use tracing::info;

use deadpool_redis::redis::AsyncTypedCommands;

use crate::CacheResult;

/// ### Redis Connection Pool
pub type Pool = deadpool_redis::Pool;

/// ### Initializing a Redis Connection Pool
pub async fn init_pool() -> Pool {
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

pub async fn get(key: impl AsRef<str>, pool: &Pool) -> CacheResult<Option<String>> {
    let key = key.as_ref();
    let mut conn = pool.get().await?;

    Ok(conn.get(key).await?)
}

pub async fn set(
    key: impl AsRef<str>,
    json: impl AsRef<str>,
    ttl: u64,
    pool: &Pool,
) -> CacheResult<()> {
    let key = key.as_ref();
    let value = json.as_ref();
    let mut conn = pool.get().await?;

    conn.set_ex(key, value, ttl).await?;
    Ok(())
}
