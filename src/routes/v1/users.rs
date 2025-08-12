use crate::error::Error;
use crate::models::api::{OptionSession, SelfUserOut, SessionOut, UserOut};
use crate::{AppState, services};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

pub fn config() -> Router<Arc<AppState>> {
    Router::new()
        .nest(
            "/me",
            Router::new()
                .route("/", get(get_user))
                .route("/sessions", get(get_user_sessions)),
        )
        .route("/{:username}", get(get_user_by_username))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    session: OptionSession,
) -> Result<Json<SelfUserOut>, Error> {
    let session = session.item.ok_or(Error::Unauthorized)?.session;

    let user = services::users::get_user(&state, session).await?;

    Ok(Json(user))
}

pub async fn get_user_sessions(
    State(state): State<Arc<AppState>>,
    session: OptionSession,
) -> Result<Json<Vec<SessionOut>>, Error> {
    let session = session.item.ok_or(Error::Unauthorized)?.session;

    let items = services::users::get_user_sessions(&state, session).await?;

    Ok(Json(items))
}

pub async fn get_user_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<UserOut>, Error> {
    let item = services::users::get_user_by_username(&state, username).await?;
    Ok(Json(item))
}
