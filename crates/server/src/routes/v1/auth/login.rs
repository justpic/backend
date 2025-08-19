use actix_web::{
    cookie::Cookie, post, web::{self, Json}, HttpRequest, HttpResponse, Responder
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use justpic_database::{models::{sessions::DbSession, users::DbUser}, postgres, redis};
use justpic_models::{api::auth::LoginDto, Validate};

use crate::{
    auth::extract::{self, SESSION_COOKIE_NAME},
    error::{Error, Result},
};

/// Login endpoint
#[utoipa::path(
    post, 
    path = "/v1/auth/login", 
    request_body = LoginDto,
    tag = "auth",
)]
#[post("/login")]
pub async fn login(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<redis::Pool>,
    payload: Json<LoginDto>,
) -> Result<impl Responder> {
    // Throw error if user try to login with active session
    extract::throw_err_if_client_has_active_session(&req, &redis_pool).await?;

    // Extract and validate request payload
    let payload = payload.into_inner();
    payload.validate()?;

    // Getting user for login
    let user_to_login = DbUser::get_by_email(payload.email, &pool).await?.ok_or(Error::InvalidCredentials)?;

    // Validating password
    if !validate_password(payload.password, user_to_login.password.clone()).await? {
        return Err(Error::InvalidCredentials);
    }

    // Getting user-agent from request headers
    let ua = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok());

    // Creating new session
    let session = DbSession::new(&user_to_login, ua);

    // Saving created session
    session.insert(&pool).await?;
    session.save_in_cache(&redis_pool).await?;

    // Creating session cookie
    let cookie = Cookie::build(SESSION_COOKIE_NAME, session.session_key)
        .path("/")
        .http_only(true)
        // .secure(true)
        .expires(session.expires)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

/// Validates the entered password by comparing the password with its hash
async fn validate_password(pwd: impl AsRef<str>, hash: impl AsRef<str>) -> Result<bool> {
    let pwd = pwd.as_ref().to_string();
    let hash = hash.as_ref().to_string();

    let task = tokio::task::spawn_blocking(move || -> Result<bool> {
        let parsed_hash = PasswordHash::new(&hash)?;
        let pwd_bytes = pwd.as_bytes();
        let res = Argon2::default().verify_password(pwd_bytes, &parsed_hash).is_ok();

        Ok(res)
    });

    task.await?
}