use axum::response::{IntoResponse, Response};
use derive_more::{Display, From};
use frog_utils::macros::HttpCode;

#[derive(Debug, Display, From, HttpCode)]
pub enum Error {
    #[code(500)]
    #[display("IO_ERROR")]
    Io(#[from] std::io::Error),

    #[code(500)]
    #[display("DATABASE_ERROR")]
    Database(#[from] crate::database::error::DatabaseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}