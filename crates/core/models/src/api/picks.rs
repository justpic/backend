use serde::Deserialize;
use utoipa::ToSchema;

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
