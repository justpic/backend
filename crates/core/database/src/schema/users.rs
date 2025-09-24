//! The `schema::user` module
//! contains structures related to the user.

use time::OffsetDateTime;
use uuid::Uuid;

use crate::types::{role::Role, visibility::Visibility};

/// Database user structure
pub struct User {
    /// User Uuid
    pub id: Uuid,

    /// User creation time
    pub created: OffsetDateTime,

    /// User name
    pub username: String,

    /// User email
    pub email: String,

    /// User password hash
    pub password: String,

    /// User role
    pub role: Role,
}

/// User profile
pub struct Profile {
    /// Profile owner uuid
    pub id: Uuid,

    /// Profile display name
    pub name: String,

    /// Profile description
    pub description: Option<String>,

    /// Profile visibility
    pub visibility: Visibility,
}
