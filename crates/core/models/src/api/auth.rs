use justpic_database::models::sessions::DbSession;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Login DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "hunter42")]
    #[validate(length(min = 8, max = 224))]
    pub password: String,
}

/// Session Api model
#[derive(Debug, Serialize)]
pub struct SessionOut {
    /// Session id
    id: Uuid,
    /// Session creation time
    created: OffsetDateTime,
    /// Sessin User Agent
    user_agent: Option<String>,
}

impl From<DbSession> for SessionOut {
    fn from(value: DbSession) -> Self {
        SessionOut {
            id: value.id,
            created: value.created,
            user_agent: value.user_agent,
        }
    }
}
