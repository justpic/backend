use actix_web::http::StatusCode;
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum Error {
    DatabaseError(#[from] justpic_database::DatabaseError),

    HashError(#[from] argon2::password_hash::Error),

    MultithreadError(#[from] tokio::task::JoinError),

    JsonError,

    InternalError,

    NotFound,

    AlreadyExists,

    ValidationError(#[from] justpic_models::ValidationError),

    Unauthorized,

    Forbidden,

    InvalidCredentionals,
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::ValidationError(..) | Error::InvalidCredentionals => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::AlreadyExists => StatusCode::CONFLICT,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
