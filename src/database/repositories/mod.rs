use super::postgres::DbPool;
use users::UserRepository;

pub mod users;

#[derive(Debug, Clone)]
pub struct Repositories {
    pub users: users::UserRepository,
}

impl Repositories {
    pub fn new(pool: &DbPool) -> Self {
        Repositories {
            users: UserRepository::new(pool.clone()),
        }
    }
}
