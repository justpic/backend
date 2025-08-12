use crate::{database::postgres::DbPool, error::Error, models::database::Session};

#[derive(Debug, Clone)]
pub struct SessionsRepository {
    pool: DbPool,
}

impl SessionsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, item: &Session) -> Result<(), Error> {
        let i = item;

        sqlx::query!(
            "
			INSERT INTO sessions (
				session_id, session_key, user_id, created, expires, os, device, user_agent
			)
			VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8
		)
		",
            i.session_id,
            i.session_key,
            i.user_id,
            i.created,
            i.expires,
            i.os,
            i.device,
            i.user_agent
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_key<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: From<Session>,
    {
        let item = sqlx::query_as!(
            Session,
            "SELECT * FROM sessions WHERE session_key = $1",
            key
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|i| i.into());

        Ok(item)
    }

    pub async fn delete_by_key(&self, key: &str) -> Result<(), Error> {
        sqlx::query!(
            "
			DELETE FROM sessions
			WHERE session_key = $1
		",
            key
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
