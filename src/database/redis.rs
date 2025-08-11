use std::sync::Arc;

use deadpool_redis::{Config, Runtime};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Redis(Arc<deadpool_redis::Pool>);

impl Redis {
    pub async fn new() -> Self {
        let pool = Arc::new(init_pool().await);
        Redis(pool)
    }

    pub fn get_json<'a, T: Deserialize<'a>>(&self, key: &str) -> Result<T, Error> {
        todo!()
    }

    pub fn set_json<'a, T: Serialize>(
        &self,
        key: &str,
        value: &'a T,
        ex: impl Into<usize>,
    ) -> Result<(), Error> {
        todo!()
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
