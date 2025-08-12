use crate::error::Error;
use crate::models::api::{OptionSession, SelfUserOut, SessionOut};
use crate::{AppState, services};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::{TypedHeader, headers};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

pub fn config() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegisterDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 64))]
    pub username: String,

    #[validate(length(min = 8, max = 224))]
    pub password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    session: OptionSession,
    Json(payload): Json<UserRegisterDto>,
) -> Result<(StatusCode, Json<SelfUserOut>), Error> {
    if session.item.is_some() {
        return Err(Error::Conflict);
    }
    payload.validate()?;

    let user = services::auth::register(&state, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 224))]
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    session: OptionSession,
    Json(payload): Json<LoginDto>,
) -> Result<(CookieJar, Json<SessionOut>), Error> {
    if session.item.is_some() {
        return Err(Error::Conflict);
    }
    payload.validate()?;

    let session = services::auth::login(&state, payload, jar, user_agent.as_str()).await?;
    Ok((session.0, Json(session.1)))
}
