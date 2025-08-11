use std::sync::Arc;

use super::postgres::DbPool;
use users::UserRepository;

pub mod users;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub users: Arc<users::UserRepository>,
}

impl Repositories {
    pub fn new(pool: &DbPool) -> Self {
        Repositories {
            users: Arc::new(UserRepository::new(pool.clone())),
        }
    }
}
