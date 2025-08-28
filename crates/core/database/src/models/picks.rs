use serde::{Deserialize, Serialize};
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

    pub status: Status,

    pub file_url: Option<String>,

    pub private: bool,
    pub ai_generated: bool,
    pub nsfw: bool,
    pub deleted: bool,
}

#[derive(FromRow, Clone, Debug)]
pub struct PickWithUser {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub source_url: Option<String>,
    pub created: OffsetDateTime,
    pub owner_id: Uuid,
    pub mimetype: String,
    pub status: Status,
    pub file_url: Option<String>,
    pub private: bool,
    pub ai_generated: bool,
    pub nsfw: bool,
    pub deleted: bool,

    pub user_id: Uuid,
    pub user_display_name: String,
    pub user_username: String,
    pub user_avatar: Option<String>,
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
            status: Status::Pending,
            file_url: None,
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

    pub async fn get_by_id_with_user(id: &Uuid, pool: &PgPool) -> Result<Option<PickWithUser>> {
        let item = sqlx::query_as!(
            PickWithUser,
            "
                SELECT 
                    p.*,
                    u.id as user_id,
                    u.display_name as user_display_name,
                    u.username as user_username,
                    u.avatar_url as user_avatar
                FROM picks p
                JOIN users u ON p.owner_id = u.id
                WHERE p.id = $1
            ",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(item)
    }

    pub async fn get_by_owner_id_with_user(
        owner_id: &Uuid,
        pool: &PgPool,
    ) -> Result<Vec<PickWithUser>> {
        let items = sqlx::query_as!(
            PickWithUser,
            "
                SELECT 
                    p.*,
                    u.id as user_id,
                    u.display_name as user_display_name,
                    u.username as user_username,
                    u.avatar_url as user_avatar
                FROM picks p
                JOIN users u ON p.owner_id = u.id
                WHERE p.owner_id = $1
            ",
            owner_id
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn get_by_owner_id(owner_id: &Uuid, pool: &PgPool) -> Result<Vec<DbPick>> {
        let items = sqlx::query_as!(DbPick, "SELECT * FROM picks WHERE owner_id = $1", owner_id)
            .fetch_all(pool)
            .await?;

        Ok(items)
    }

    pub async fn set_status(&self, status: Status, pool: &PgPool) -> Result<()> {
        let status = status.to_string();
        sqlx::query!(
            "
            UPDATE picks
            SET status = $1
            WHERE id = $2
        ",
            &status,
            self.id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn set_file_url(&self, url: impl AsRef<str>, pool: &PgPool) -> Result<()> {
        let url = url.as_ref();
        sqlx::query!(
            "UPDATE picks
            SET file_url = $1
            WHERE id = $2",
            url,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn remove(&self, pool: &PgPool) -> Result<()> {
        Self::remove_by_id(&self.id, pool).await?;
        Ok(())
    }

    pub async fn remove_by_id(id: &Uuid, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "
					UPDATE picks
					SET deleted = true
					WHERE id = $1
				",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Processing,
    Ready,
    Failed,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        let raw = value.as_str();
        match raw {
            "processing" => Status::Processing,
            "ready" => Status::Ready,
            "failed" => Status::Failed,
            _ => Status::Pending,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Status {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Status::Pending => "pending",
            Status::Processing => "processing",
            Status::Ready => "ready",
            Status::Failed => "failed",
        }
    }
}
