use uuid::Uuid;

use crate::{database::postgres::DbPool, error::Error, models::database::User};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, item: &User) -> Result<(), Error> {
        let i = item;

        sqlx::query!(
            "
			INSERT INTO users (
				id, created, username, email, 
				password_hash, role, avatar_url, email_confirmed, nsfw_allowed
			)
			VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9	
		)
		",
            i.id,
            i.created,
            i.username,
            i.email,
            i.password_hash,
            i.role.as_str(),
            i.avatar_url,
            i.email_confirmed,
            i.nsfw_allowed
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn check_exist(&self, username: &str, email: &str) -> Result<bool, Error> {
        let i = sqlx::query_scalar!(
            "SELECT id FROM users 
            WHERE LOWER(username) = $1 OR LOWER(email) = $2",
            username.to_lowercase(),
            email.to_lowercase()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(!i.is_empty())
    }

    pub async fn get_by_id<T>(&self, id: &Uuid) -> Result<Option<T>, Error>
    where
        T: From<User>,
    {
        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .map(|i| i.into());

        Ok(item)
    }

    pub async fn get_by_username<T>(&self, username: &str) -> Result<Option<T>, Error>
    where
        T: From<User>,
    {
        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?
            .map(|i| i.into());

        Ok(item)
    }

    pub async fn get_by_email<T>(&self, email: &str) -> Result<Option<T>, Error>
    where
        T: From<User>,
    {
        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?
            .map(|i| i.into());

        Ok(item)
    }
}
