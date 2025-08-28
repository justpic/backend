use crate::DbResult;
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;
use tracing::log::debug;
use uuid::Uuid;

use super::{roles::Role, sessions::DbSession};

type Result<T> = DbResult<T>;

/// ## User database model
#[derive(FromRow, Clone, Debug)]
pub struct DbUser {
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

impl DbUser {
    /// Create a new [`DbUser`]
    pub fn new<T>(email: T, display_name: T, password_hash: T, username: T) -> DbUser
    where
        T: Into<String>,
    {
        let username = process_username(username.into());
        debug!("Creating new user [username: {}]", username);

        DbUser {
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

    /// Insert [`DbUser`] into database
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

    /// Get [`DbUser`] by ID
    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<DbUser>> {
        debug!("Fetching user by id [uid: {}]", id);

        let item = sqlx::query_as!(DbUser, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    /// Get [`DbUser`] by Username
    pub async fn get_by_username(
        username: impl AsRef<str>,
        pool: &PgPool,
    ) -> Result<Option<DbUser>> {
        let username = process_username(username.as_ref());
        debug!("Fetching user by username [username: {}]", username);

        let item = sqlx::query_as!(DbUser, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    /// Get [`DbUser`] by Email
    pub async fn get_by_email(email: impl AsRef<str>, pool: &PgPool) -> Result<Option<DbUser>> {
        let email = email.as_ref();
        debug!("Fetching user by email [email: {}]", email);

        let item = sqlx::query_as!(DbUser, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    /// Get [`DbUser`] by [`DbSession`]
    pub async fn get_by_session(session: &DbSession, pool: &PgPool) -> Result<Option<DbUser>> {
        let uid = &session.user_id;

        DbUser::get_by_id(uid, pool).await
    }

    pub async fn remove(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "
                DELETE FROM users
                WHERE id = $1
            ",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

/// Converts the username to the required format
///
/// E.g. `John Doe 42` -> `john_doe_42`
fn process_username(raw: impl AsRef<str>) -> String {
    let unprocessed = raw.as_ref();

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

        let user = DbUser::new(email, d_name, p_hash, username);

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
