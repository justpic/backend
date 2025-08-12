use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::database::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOut {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub username: String,
    pub avatar_url: Option<String>,
}

impl From<User> for UserOut {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            created: value.created,
            username: value.username,
            avatar_url: value.avatar_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelfUserOut {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub email_confirmed: bool,
    pub nsfw_allowed: bool,
}

impl From<User> for SelfUserOut {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            created: value.created,
            username: value.username,
            email: value.email,
            avatar_url: value.avatar_url,
            email_confirmed: value.email_confirmed,
            nsfw_allowed: value.nsfw_allowed,
        }
    }
}
