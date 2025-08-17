use actix_web::{
    cookie::{time::Duration, Cookie}, post, web::{self, Json}, HttpMessage, HttpRequest, HttpResponse, Responder
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use justpic_database::{models::{sessions::{DbSession, SESSION_TTL_INT}, users::DbUser}, postgres, redis};
use justpic_models::{api::auth::LoginDto, Validate};

use crate::{
    auth::extract::SESSION_COOKIE_NAME,
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
    let payload = payload.into_inner();
    payload.validate()?;

    let user_to_login = DbUser::get_by_email(payload.email, &pool).await?.ok_or(Error::InvalidCredentionals)?;

    if !validate_password(payload.password, user_to_login.password.clone()).await? {
        return Err(Error::InvalidCredentionals);
    }

    let ua = req.headers()
        .get("User-Agent")
        .map(|v| v.to_str().ok())
        .ok_or(Error::Forbidden)?;

    let session = DbSession::new(&user_to_login, ua);

    session.insert(&pool).await?;
    session.save_in_cache(&redis_pool).await?;

    let cookie = Cookie::build(SESSION_COOKIE_NAME, session.session_key)
        .path("/")
        .http_only(true)
        .expires(session.expires)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

/// Validates the entered password by comparing the password with its hash
async fn validate_password<T>(pwd: T, hash: T) -> Result<bool>
where 
    T:Into<String>
{
    let pwd: String = pwd.into();
    let hash: String = hash.into();

    let task = tokio::task::spawn_blocking(move || -> Result<bool> {
        let parsed_hash = PasswordHash::new(&hash)?;
        let pwd_bytes = pwd.as_bytes();
        let res = Argon2::default().verify_password(pwd_bytes, &parsed_hash).is_ok();

        Ok(res)
    });

    task.await?
}