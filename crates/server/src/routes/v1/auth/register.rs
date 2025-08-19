use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, HttpResponse};
use argon2::Algorithm::Argon2id;
use argon2::Version::V0x13;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, Params, PasswordHasher};

use justpic_database::models::users::DbUser;
use justpic_database::{postgres, redis};

use justpic_models::Validate;
use justpic_models::api::users::RegisterDto;

use crate::auth::extract;
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
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<redis::Pool>,
    payload: Json<RegisterDto>,
) -> Result<HttpResponse> {
    // Throw error if user try to register new account with active session
    extract::throw_err_if_client_has_active_session(&req, &redis_pool).await?;

    let payload = payload.into_inner();
    payload.validate()?;

    // Todo: заменить на один запрос к базе
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
async fn hash_password(raw: impl AsRef<str>) -> Result<String> {
    let password_str = raw.as_ref().to_string();
    let task = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(32 * 1024, 3, 2, None).unwrap();
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

        let hashed = hash_password(test_pass).await.unwrap();

        assert!(test_pass != hashed)
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
