use justpic_database::models::users::DbUser;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSelfOut {
    /// User data
    #[serde(flatten)]
    pub user: UserOut,

    /// Email
    pub email: String,
}

impl From<DbUser> for UserSelfOut {
    fn from(value: DbUser) -> Self {
        UserSelfOut {
            email: value.email.clone(),
            user: UserOut::from(value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserOut {
    /// Unique user id
    pub id: Uuid,

    /// Display name
    pub display_name: String,

    /// Username
    pub username: String,

    /// Avatar url
    pub avatar_url: Option<String>,

    /// Banner url
    pub banner_url: Option<String>,

    /// Created at
    pub created: OffsetDateTime,
}

impl From<DbUser> for UserOut {
    fn from(value: DbUser) -> Self {
        UserOut {
            id: value.id,
            display_name: value.display_name,
            username: value.username,
            avatar_url: value.avatar_url,
            banner_url: value.banner_url,
            created: value.created,
        }
    }
}

/// User register DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct RegisterDto {
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

    #[schema(example = "John Doe")]
    #[validate(length(min = 4, max = 128))]
    pub display_name: String,
}
