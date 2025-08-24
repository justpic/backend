use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub private: bool,
    pub ai_generated: bool,
    pub nsfw: bool,
}

impl UploadDto {
    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice::<Self>(bytes).ok()
    }
}
