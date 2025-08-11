use sessions::SessionsRepository;
use users::UserRepository;

use super::postgres::DbPool;

pub mod sessions;
pub mod users;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub users: UserRepository,
    pub sessions: SessionsRepository,
}

impl Repositories {
    pub fn new(pool: &DbPool) -> Self {
        Repositories {
            users: UserRepository::new(pool.clone()),
            sessions: SessionsRepository::new(pool.clone()),
        }
    }
}
