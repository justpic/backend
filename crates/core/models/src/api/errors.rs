use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorOut {
    code: u16,
    message: String,
}

impl ErrorOut {
    pub fn new(code: u16, message: String) -> ErrorOut {
        ErrorOut { code, message }
    }
}
