use sqlx::prelude::{FromRow, Type};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created: OffsetDateTime,

    pub username: String,
    pub email: String,
    pub password_hash: String,

    pub role: Role,

    pub avatar_url: Option<String>,

    pub email_confirmed: bool,
    pub nsfw_allowed: bool,
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "test", rename_all = "lowercase")]
pub enum Role {
    Regular,
    Moderator,
    Admin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        Self::from_string(&value)
    }
}

impl Role {
    pub fn from_string(string: &str) -> Role {
        match string {
            "admin" => Role::Admin,
            "moderator" => Role::Moderator,
            _ => Role::Regular,
        }
    }

    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Role::Regular => "regular",
            Role::Moderator => "moderator",
            Role::Admin => "admin",
        }
    }
}
