use uuid::Uuid;

use crate::{
    database::{error::DatabaseError, postgres::DbPool},
    models::database::User,
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn insert<T>(&self, item: T) -> Result<(), DatabaseError>
    where
        T: Into<User>,
    {
        let i: User = item.into();

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
        todo!()
    }

    pub async fn get_by_id<T>(&self, id: &Uuid) -> Result<Option<T>, DatabaseError>
    where
        T: From<User>,
    {
        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .map(|i| i.into());

        Ok(item)
    }

    pub async fn get_by_username<T>(&self, username: &str) -> Result<Option<T>, DatabaseError>
    where
        T: From<User>,
    {
        let item = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?
            .map(|i| i.into());

        Ok(item)
    }
}
