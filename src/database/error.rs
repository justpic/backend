use derive_more::{Display, From};

#[derive(Debug, From, Display)]
pub enum DatabaseError {
    Sqlx(#[from] sqlx::error::Error),

    RedisPool(#[from] deadpool_redis::PoolError),

    Redis(#[from] redis::RedisError),
}
