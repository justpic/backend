use justpic_database::models::cards::{Card, CardWithUser, Status};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

/// Upload 'card' Dto
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCardRequest {
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

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
pub struct CardOwner {
    pub id: Uuid,
    #[schema(example = "johndoe")]
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CardResponse {
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
    #[schema(example = "abc.jpg")]
    pub file_url: Option<String>,
    #[schema(nullable = false)]
    pub owner: Option<CardOwner>,
}

impl From<Card> for CardResponse {
    fn from(value: Card) -> Self {
        CardResponse {
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
            file_url: value.file_url,
            owner: None,
        }
    }
}

impl From<CardWithUser> for CardResponse {
    fn from(value: CardWithUser) -> Self {
        let owner = CardOwner {
            id: value.user_id,
            username: value.user_username,
            avatar: value.user_avatar,
        };

        CardResponse {
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
            file_url: value.file_url,
            owner: Some(owner),
        }
    }
}
