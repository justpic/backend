use actix_web::web::Json;
use actix_web::{HttpResponse, post, web};
use argon2::Algorithm::Argon2id;
use argon2::Version::V0x13;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, Params, PasswordHasher};

use justpic_database::models::users::DbUser;
use justpic_database::postgres;

use justpic_models::Validate;
use justpic_models::api::users::RegisterDto;

use crate::error::{Error, Result};

/// Register endpoint
#[utoipa::path(
	post, 
	path = "/v1/auth/register", 
	request_body = RegisterDto, 
	tag = "auth",
	responses(
        (status = 201, description = "User created"),
        (status = 409, description = "Already exists")
    )
)]
#[post("/register")]
pub async fn register(
    pool: web::Data<postgres::Pool>,
    payload: Json<RegisterDto>,
) -> Result<HttpResponse> {
    let payload = payload.into_inner();
    payload.validate()?;

    if DbUser::get_by_username(&payload.username, &pool).await?.is_some()
        || DbUser::get_by_email(&payload.email, &pool).await?.is_some()
    {
        return Err(Error::AlreadyExists);
    }

    let password_hash = hash_password(payload.password).await?;

    let new_user = DbUser::new(
        payload.email,
        payload.display_name,
        password_hash,
        payload.username,
    );

    new_user.insert(&pool).await?;

    Ok(HttpResponse::Created().finish())
}

/// Hash user password using the Argon2-algorithm
async fn hash_password<T>(raw: T) -> Result<String>
where
    T: Into<String>,
{
    let password_str = raw.into();
    let task = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(10 * 1024, 2, 1, None).unwrap();
        let argon2 = Argon2::new(Argon2id, V0x13, params);

        argon2
            .hash_password(password_str.as_bytes(), &salt)
            .map(|v| v.to_string())
    });

    Ok(task.await??)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hashing() {
        let test_pass = "hunter42";

        hash_password(test_pass).await.unwrap();
    }

    #[test]
    fn test_request_body_dto_validator() {
        let test_dto = RegisterDto {
            email: "invalid email!!".to_string(),
            password: "abc".to_string(),
            username: ":)".to_string(),
            display_name: "Invalid user".to_string(),
        };

        assert!(test_dto.validate().is_err());
    }
}
