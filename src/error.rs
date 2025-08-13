use actix_web::{HttpResponse, http::StatusCode};
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display("DATABASE_ERROR")]
    Sqlx(#[from] sqlx::Error),

    #[display("JSON_PARSE_ERROR")]
    Json(#[from] serde_json::Error),

    #[display("VALIDATION_ERROR")]
    ValidationError,

    #[display("BAD_REQUEST")]
    BadRequest,

    #[display("UNAUTHORIZED")]
    Unauthorized,

    #[display("INVALID_AUTH_CREDENTIALS")]
    InvalidAuthCredentials,

    #[display("INVALID_SESSION")]
    InvalidSession,

    #[display("FORBIDDEN")]
    Forbidden,

    #[display("NOT_FOUND")]
    NotFound,

    #[display("ALREADY_EXISTS")]
    AlreadyExists,
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::AlreadyExists => StatusCode::CONFLICT,
            Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::Forbidden => StatusCode::FORBIDDEN,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
