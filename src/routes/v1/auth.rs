use crate::error::Error;
use crate::models::api::{SelfUserOut, UserRegisterDto};
use crate::{AppState, services};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use validator::Validate;

pub fn config() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserRegisterDto>,
) -> Result<(StatusCode, Json<SelfUserOut>), Error> {
    payload.validate()?;

    let user = services::auth::register(&state, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}
