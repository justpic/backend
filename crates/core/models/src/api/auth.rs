use justpic_database::models::sessions::DbSession;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Login DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "hunter42")]
    #[validate(length(min = 8, max = 224))]
    pub password: String,
}

/// Session Api model
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    /// Session id
    #[schema(example = "20c14e9f-bb96-4690-a446-ba523327d138")]
    id: Uuid,
    /// Session creation time
    #[schema(example = "2025-08-25 17:26:54.885438+00")]
    created: OffsetDateTime,
    /// Sessin User Agent
    #[schema(example = "Mozilla/5.0 (X11; Linux x86_64; rv:141.0) Firefox/141.0")]
    user_agent: Option<String>,
}

impl From<DbSession> for SessionResponse {
    fn from(value: DbSession) -> Self {
        SessionResponse {
            id: value.id,
            created: value.created,
            user_agent: value.user_agent,
        }
    }
}
