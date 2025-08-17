use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// User register DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct RegisterDto {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "hunter42")]
    #[validate(length(min = 8, max = 224))]
    pub password: String,

    // todo!: add whitespaces and symbols validation
    #[schema(example = "JohnDoe")]
    #[validate(length(min = 3, max = 128))]
    pub username: String,

    #[schema(example = "John Doe")]
    #[validate(length(min = 4, max = 128))]
    pub display_name: String,
}
