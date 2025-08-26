use justpic_database::models::picks::{DbPick, PickWithUser, Status};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

/// Upload 'pick' Dto
#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadRequest {
    #[schema(example = "Cute kitty!")]
    pub title: Option<String>,
    #[schema(example = "Awwwwww!")]
    pub description: Option<String>,
    #[schema(example = "pinterest.com")]
    pub source: Option<String>,
    #[schema(example = false)]
    pub private: bool,
    #[schema(example = false)]
    pub ai_generated: bool,
    #[schema(example = false)]
    pub nsfw: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PickOwner {
    pub id: Uuid,
    #[schema(example = "John Doe")]
    pub display_name: String,
    #[schema(example = "johndoe")]
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PickOut {
    pub id: Uuid,
    #[schema(example = "Cute cat!")]
    pub title: Option<String>,
    #[schema(example = "Aww!")]
    pub description: Option<String>,
    #[schema(example = "pinterest.com")]
    pub source_url: Option<String>,
    pub created: OffsetDateTime,
    #[schema(example = "image/png")]
    pub mimetype: String,
    #[schema(value_type = String, example = "ready")]
    pub status: Status,
    #[schema(example = false)]
    pub private: bool,
    #[schema(example = false)]
    pub ai_generated: bool,
    #[schema(example = false)]
    pub nsfw: bool,
    #[schema(example = false)]
    pub deleted: bool,
    #[schema(example = "/v1/picks/abc/file")]
    pub file_url: String,
    pub owner: Option<PickOwner>,
}

impl From<DbPick> for PickOut {
    fn from(value: DbPick) -> Self {
        PickOut {
            id: value.id,
            title: value.title,
            description: value.description,
            source_url: value.source_url,
            created: value.created,
            mimetype: value.mimetype,
            status: value.status,
            private: value.private,
            ai_generated: value.ai_generated,
            nsfw: value.nsfw,
            deleted: value.deleted,
            file_url: format!("/v1/picks/{}/file", value.id),
            owner: None,
        }
    }
}

impl From<PickWithUser> for PickOut {
    fn from(value: PickWithUser) -> Self {
        let owner = PickOwner {
            id: value.user_id,
            display_name: value.user_display_name,
            username: value.user_username,
            avatar: value.user_avatar,
        };

        PickOut {
            id: value.id,
            title: value.title,
            description: value.description,
            source_url: value.source_url,
            created: value.created,
            mimetype: value.mimetype,
            status: value.status,
            private: value.private,
            ai_generated: value.ai_generated,
            nsfw: value.nsfw,
            deleted: value.deleted,
            file_url: format!("/v1/picks/{}/file", value.id),
            owner: Some(owner),
        }
    }
}
