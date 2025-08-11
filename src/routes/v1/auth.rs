use crate::error::Error;
use crate::models::api::SelfUserOut;
use crate::{AppState, services};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

pub fn config() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register))
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
    Json(payload): Json<UserRegisterDto>,
) -> Result<(StatusCode, Json<SelfUserOut>), Error> {
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
