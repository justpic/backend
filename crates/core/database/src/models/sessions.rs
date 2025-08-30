use crate::{DbResult, models::users::DbUser};
use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

type Result<T> = DbResult<T>;

pub const SESSION_TTL_INT: u8 = 28;
pub const CACHE_PREFIX: &str = "session";

/// ## Session database model
#[derive(FromRow, Clone, Debug, Deserialize, Serialize)]
pub struct DbSession {
    /// Unique session id
    pub id: Uuid,

    /// Session owner id
    pub user_id: Uuid,

    /// Unique session key
    pub session_key: String,

    /// Created at
    pub created: OffsetDateTime,

    /// Expires at
    pub expires: OffsetDateTime,

    /// Session user agent
    pub user_agent: Option<String>,
}

impl DbSession {
    /// Create a new [`DbSession`]
    pub fn new<T>(user: &DbUser, user_agent: Option<T>) -> DbSession
    where
        T: Into<String>,
    {
        let created = OffsetDateTime::now_utc();
        let expires = created + Duration::days(SESSION_TTL_INT as i64);

        DbSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            session_key: generate_session_key(),
            created,
            expires,
            user_agent: user_agent.map(|v| v.into()),
        }
    }

    /// Insert [`DbSession`] into database
    pub async fn insert(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "
            INSERT INTO sessions (
                id, user_id, session_key, created, 
                expires, user_agent
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6
             )
        ",
            self.id,
            self.user_id,
            self.session_key,
            self.created,
            self.expires,
            self.user_agent,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get [`DbSession`] by key
    pub async fn get_by_key(key: impl AsRef<str>, pool: &PgPool) -> Result<Option<DbSession>> {
        let item = sqlx::query_as!(
            DbSession,
            "SELECT * FROM sessions WHERE session_key = $1",
            key.as_ref()
        )
        .fetch_optional(pool)
        .await?;

        Ok(item)
    }

    pub async fn get_by_owner_id(
        owner_id: impl AsRef<Uuid>,
        pool: &PgPool,
    ) -> Result<Vec<DbSession>> {
        let items = sqlx::query_as!(
            DbSession,
            "SELECT * FROM sessions
            WHERE user_id = $1
            ",
            owner_id.as_ref()
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    /// Remove [`DbSession`] from db
    pub async fn remove(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", self.id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

fn generate_session_key() -> String {
    let mut buf = [0u8; 64];
    OsRng.fill_bytes(&mut buf);
    BASE64_STANDARD.encode(buf)
}
