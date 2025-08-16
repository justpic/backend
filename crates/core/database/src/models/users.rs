use crate::DbResult;
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;
use tracing::log::debug;
use uuid::Uuid;

use super::roles::Role;

type Result<T> = DbResult<T>;

/// ## User model
#[derive(FromRow, Clone, Debug)]
pub struct User {
    /// Unique user id
    pub id: Uuid,

    /// Email
    pub email: String,

    /// Password hash
    pub password: String,

    /// Display name
    pub display_name: String,

    /// Username
    pub username: String,

    /// Avatar url
    pub avatar_url: Option<String>,

    /// Banner url
    pub banner_url: Option<String>,

    /// User role
    pub role: Role,

    /// Created at
    pub created: OffsetDateTime,
}

impl User {
    /// Create a new [`User`]
    pub fn new<T>(email: T, display_name: T, password_hash: T, username: T) -> User
    where
        T: Into<String>,
    {
        let username = process_username(username);
        debug!("Creating new user [username: {}]", username);

        User {
            id: Uuid::new_v4(),
            email: email.into(),
            password: password_hash.into(),
            display_name: display_name.into(),
            username,
            avatar_url: None,
            banner_url: None,
            role: Role::Regular,
            created: OffsetDateTime::now_utc(),
        }
    }

    /// Insert [`User`] into database
    pub async fn insert(&self, pool: &PgPool) -> Result<()> {
        debug!("Inserting user in db [uid: {}]", self.id);

        sqlx::query!(
            "
            INSERT INTO users (
                id, email, password, display_name, 
                username, avatar_url, banner_url, created
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6, $7, $8
             )
        ",
            self.id,
            self.email,
            self.password,
            self.display_name,
            self.username,
            self.avatar_url,
            self.banner_url,
            self.created
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get [`User`] by ID
    pub async fn get_by_id<T>(id: T, pool: &PgPool) -> Result<Option<User>>
    where
        T: Into<Uuid>,
    {
        let id: Uuid = id.into();
        debug!("Fetching user by id [uid: {}]", id);

        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    /// Get [`User`] by Username
    pub async fn get_by_username<T>(username: T, pool: &PgPool) -> Result<Option<User>>
    where
        T: Into<String>,
    {
        let username = process_username(username);
        debug!("Fetching user by username [username: {}]", username);

        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    /// Get [`User`] by Email
    pub async fn get_by_email<T>(email: T, pool: &PgPool) -> Result<Option<User>>
    where
        T: Into<String>,
    {
        let email = email.into();
        debug!("Fetching user by email [email: {}]", email);

        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }
}

/// Converts the username to the required format
///
/// E.g. `John Doe 42` -> `john_doe_42`
fn process_username<T>(raw: T) -> String
where
    T: Into<String>,
{
    let unprocessed: String = raw.into();

    unprocessed
        .split_whitespace()
        .filter(|v| !v.is_empty())
        .collect::<Vec<&str>>()
        .join("_")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let email = "test@example.com";
        let username = "john_doe";
        let d_name = "John Doe";
        let p_hash = "hunter42";

        let user = User::new(email, d_name, p_hash, username);

        assert_eq!(user.username, process_username(username));

        assert_eq!(user.email, email);
    }

    #[test]
    fn test_username_processer() {
        assert_eq!(process_username("John Doe 42"), "john_doe_42");

        assert_eq!(process_username(" J D 4 "), "j_d_4");

        // username chars validation is done in DTO
        assert_eq!(process_username("  "), "");
    }
}
