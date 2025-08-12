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
    #[display("INTERNAL_SERVER_ERROR")]
    Io(#[from] std::io::Error),

    #[code(500)]
    #[display("DATABASE_ERROR")]
    Sqlx(#[from] sqlx::error::Error),

    #[code(500)]
    #[display("DATABASE_ERROR")]
    RedisPool(#[from] deadpool_redis::PoolError),

    #[code(500)]
    #[display("DATABASE_ERROR")]
    Redis(#[from] redis::RedisError),

    #[code(400)]
    #[display("VALIDATION_ERROR")]
    Validation(#[from] validator::ValidationErrors),

    #[code(500)]
    #[display("HASHING_ERROR")]
    Hash(#[from] argon2::password_hash::Error),

    #[code(409)]
    #[display("CONFLICT")]
    Conflict,

    #[code(500)]
    #[display("JSON_PARSE_ERROR")]
    Serde(#[from] serde_json::Error),

    #[code(400)]
    #[display("INVALID_AUTHENTICATION_CREDENTIALS")]
    InvalidCredentials,

    #[code(500)]
    #[display("INTERNAL_SERVER_ERROR")]
    Multithread(#[from] tokio::task::JoinError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.http_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let json = ErrorOut::new(self.http_code(), self.to_string());

        (status_code, Json(json)).into_response()
    }
}
