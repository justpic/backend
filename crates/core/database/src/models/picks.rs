use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DbResult;

type Result<T> = DbResult<T>;

/// ### "Pick" database model
#[derive(FromRow, Clone, Debug, Serialize)]
pub struct DbPick {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub source_url: Option<String>,
    pub created: OffsetDateTime,
    pub owner_id: Uuid,
    pub mimetype: String,
    pub private: bool,
    pub ai_generated: bool,
    pub nsfw: bool,
    pub deleted: bool,
}

impl DbPick {
    /// Create a new [`DbPick`]
    pub fn new<T>(
        id: Uuid,
        title: Option<T>,
        description: Option<T>,
        source_url: Option<T>,
        owner_id: Uuid,
        mimetype: T,
        private: bool,
        ai_generated: bool,
        nsfw: bool,
    ) -> DbPick
    where
        T: Into<String>,
    {
        DbPick {
            id,
            title: title.map(|v| v.into()),
            description: description.map(|v| v.into()),
            source_url: source_url.map(|v| v.into()),
            created: OffsetDateTime::now_utc(),
            owner_id,
            mimetype: mimetype.into(),
            private,
            ai_generated,
            nsfw,
            deleted: false,
        }
    }

    /// Insert [`DbPick`] into database
    pub async fn insert(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "
					INSERT INTO picks (
						id, title, description, source_url, created,
						owner_id, mimetype, private, ai_generated, nsfw, deleted
				) VALUES (
					$1, $2, $3, $4, $5,
					$6, $7, $8, $9, $10, $11
				 )
				",
            self.id,
            self.title,
            self.description,
            self.source_url,
            self.created,
            self.owner_id,
            self.mimetype,
            self.private,
            self.ai_generated,
            self.nsfw,
            self.deleted
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<DbPick>> {
        let item = sqlx::query_as!(DbPick, "SELECT * FROM picks WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;

        Ok(item)
    }

    pub async fn get_by_owner_id(owner_id: &Uuid, pool: &PgPool) -> Result<Vec<DbPick>> {
        let items = sqlx::query_as!(DbPick, "SELECT * FROM picks WHERE owner_id = $1", owner_id)
            .fetch_all(pool)
            .await?;

        Ok(items)
    }

    pub async fn remove(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "
					UPDATE picks
					SET deleted = true
					WHERE id = $1
				",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
