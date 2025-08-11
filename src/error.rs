use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::{Display, From};
use frog_utils::macros::HttpCode;

use crate::models::api::ErrorOut;

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
        let status_code =
            StatusCode::from_u16(self.http_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let json = ErrorOut::new(self.http_code(), self.to_string());

        (status_code, Json(json)).into_response()
    }
}
