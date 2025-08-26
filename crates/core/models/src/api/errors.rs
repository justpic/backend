use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = 500)]
    code: u16,
    #[schema(example = "UNDEFINED_ERROR")]
    message: String,
}

impl ErrorResponse {
    pub fn new(code: u16, message: String) -> ErrorResponse {
        ErrorResponse { code, message }
    }
}
