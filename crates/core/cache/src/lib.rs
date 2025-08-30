use derive_more::{Display, From};

pub(crate) mod cache;
pub(crate) mod redis;

#[derive(Debug, From, Display)]
pub enum CacheError {
    RedisPoolError(#[from] deadpool_redis::PoolError),
    RedisError(#[from] deadpool_redis::redis::RedisError),

    CacheJsonError(#[from] serde_json::Error),
}

type CacheResult<T> = Result<T, CacheError>;

pub use cache::{cache_wrapper, get_from_cache, remove_from_cache, save_in_cache};
pub use redis::{Pool, init_pool};
