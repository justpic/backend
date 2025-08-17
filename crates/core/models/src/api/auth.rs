use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Login DTO
#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "hunter42")]
    #[validate(length(min = 8, max = 224))]
    pub password: String,
}
