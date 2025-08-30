pub mod postgres;

pub mod models;

type DbResult<T> = Result<T, DatabaseError>;

pub type DatabaseError = sqlx::Error;
