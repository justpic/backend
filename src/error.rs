use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum Error {
    #[display("IO_ERROR")]
    Io(#[from] std::io::Error),

    #[display("DATABASE_ERROR")]
    Database(#[from] crate::database::error::DatabaseError),
}
