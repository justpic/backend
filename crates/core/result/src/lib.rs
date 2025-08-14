use actix_web::http::StatusCode;
use derive_more::{Display, From};

#[derive(Debug, From, Display)]
pub enum Error {
    // General errors
    #[display("DATABASE_ERROR")]
    DatabaseError(#[from] sqlx::Error),

    #[display("JSON_PARSE_ERROR")]
    JsonError(#[from] serde_json::Error),

    #[display("INTERNAL_SERVER_ERROR")]
    InternalError,

    #[display("NOT_FOUND")]
    NotFound,

    #[display("VALIDATION_ERROR: {_0}")]
    ValidationError(#[from] validator::ValidationErrors),
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::ValidationError(..) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
