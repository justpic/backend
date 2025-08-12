use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

    pub const fn as_str<'a>(&self) -> &'a str {
        match self {
            Role::Regular => "regular",
            Role::Moderator => "moderator",
            Role::Admin => "admin",
        }
    }

    pub const fn is_moderator(&self) -> bool {
        match self {
            Role::Moderator | Role::Admin => true,
            _ => false,
        }
    }

    pub const fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Role enum tests
    #[test]
    fn test_role_checking() {
        let test_role = Role::Regular;

        // Moderator checking test for regular
        assert_eq!(test_role.is_moderator(), false);

        // Admin checking test for regular
        assert_eq!(test_role.is_admin(), false);

        let test_moder_role = Role::Moderator;

        // Moderator checking test for moder
        assert_eq!(test_moder_role.is_moderator(), true);

        // Admin checking test for moder
        assert_eq!(test_moder_role.is_admin(), false);

        let test_admin_role = Role::Admin;

        // Moderator checking test for admin
        assert_eq!(test_admin_role.is_moderator(), true);

        // Admin checking test for admin
        assert_eq!(test_admin_role.is_admin(), true);
    }

    #[test]
    fn test_role_from_string() {
        let role_str = "regular";

        assert!(matches!(Role::from_string(role_str), Role::Regular))
    }
}
