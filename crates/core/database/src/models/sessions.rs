use crate::{DbResult, models::users::DbUser};
use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{Engine, prelude::BASE64_STANDARD};
use deadpool_redis::redis::AsyncTypedCommands;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

type Result<T> = DbResult<T>;

pub const SESSION_TTL_INT: u8 = 28;
pub const CACHE_PREFIX: &str = "session:";

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

    /// Save [`DbSession`] in cache and return cache key
    pub async fn save_in_cache(&self, redis_pool: &deadpool_redis::Pool) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        let mut conn = redis_pool.get().await?;

        let key = [CACHE_PREFIX, &self.session_key].join(":");

        conn.set_ex(&key, json, SESSION_TTL_INT as u64 * 24 * 3600)
            .await?;

        Ok(key)
    }

    /// Get [`DbSession`] from cache by session-key
    pub async fn get_from_cache<T>(
        key: T,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<Option<DbSession>>
    where
        T: Into<String>,
    {
        let mut conn = redis_pool.get().await?;

        let key = [CACHE_PREFIX, &key.into()].join(":");

        Ok(match conn.get(&key).await? {
            Some(json) => Some(serde_json::from_str::<DbSession>(&json)?),
            None => None,
        })
    }

    /// Get [`DbSession`] by key
    pub async fn get_by_key<T>(key: T, pool: &PgPool) -> Result<Option<DbSession>>
    where
        T: Into<String>,
    {
        let key = key.into();

        let item = sqlx::query_as!(
            DbSession,
            "SELECT * FROM sessions WHERE session_key = $1",
            key
        )
        .fetch_optional(pool)
        .await?;

        Ok(item)
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
