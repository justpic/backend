use derive_more::{Display, From};

pub mod postgres;
pub mod redis;

pub mod models;

type DbResult<T> = Result<T, DatabaseError>;

#[derive(Debug, From, Display)]
pub enum DatabaseError {
    SqlxError(#[from] sqlx::Error),
}
