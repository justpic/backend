use std::sync::Arc;

use deadpool_redis::{Config, Runtime};
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Redis(Arc<deadpool_redis::Pool>);

impl Redis {
    pub async fn new() -> Self {
        let pool = Arc::new(init_pool().await);
        Redis(pool)
    }

    pub async fn get_json<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        let mut conn = self.0.get().await?;
        let raw: Option<String> = conn.get(key).await?;

        Ok(match raw {
            Some(v) => serde_json::from_str::<T>(&v).ok(),
            None => None,
        })
    }

    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ex: u64) -> Result<(), Error> {
        let json = serde_json::to_string(value)?;
        let mut conn = self.0.get().await?;

        Ok(conn.set_ex(key, &json, ex).await?)
    }
}

pub async fn init_pool() -> deadpool_redis::Pool {
    let url = dotenvy::var("REDIS_URL").expect(".env does not contain 'REDIS_URL'");

    Config::from_url(url)
        .builder()
        .expect("Error creating redis connection pool")
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Redis connection failed")
}
