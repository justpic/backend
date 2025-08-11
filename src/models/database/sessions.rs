use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Session {
    pub session_id: Uuid,
    pub session_key: String,

    pub user_id: Uuid,

    pub created: OffsetDateTime,
    pub expires: OffsetDateTime,

    pub os: Option<String>,
    pub device: Option<String>,
    pub user_agent: Option<String>,
}
