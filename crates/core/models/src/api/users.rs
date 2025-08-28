use justpic_database::models::users::DbUser;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct UserSelfResponse {
    /// User data
    #[serde(flatten)]
    pub user: UserResponse,

    /// Email
    #[schema(example = "user@example.com")]
    pub email: String,
}

impl From<DbUser> for UserSelfResponse {
    fn from(value: DbUser) -> Self {
        UserSelfResponse {
            email: value.email.clone(),
            user: UserResponse::from(value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct UserResponse {
    /// Unique user id
    pub id: Uuid,

    /// Username
    #[schema(example = "johndoe")]
    pub username: String,

    /// Avatar url
    #[schema(example = "in process")]
    pub avatar_url: Option<String>,

    /// Banner url
    #[schema(example = "in process")]
    pub banner_url: Option<String>,

    /// Created at
    pub created: OffsetDateTime,
}

impl From<DbUser> for UserResponse {
    fn from(value: DbUser) -> Self {
        UserResponse {
            id: value.id,
            username: value.username,
            avatar_url: value.avatar_url,
            banner_url: value.banner_url,
            created: value.created,
        }
    }
}

/// User register DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "hunter42")]
    #[validate(length(min = 8, max = 224))]
    pub password: String,

    // todo!: add whitespaces and symbols validation
    #[schema(example = "JohnDoe")]
    #[validate(length(min = 3, max = 128))]
    pub username: String,
}
