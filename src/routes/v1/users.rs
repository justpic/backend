use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::{get, post};
use crate::{services, AppState};
use crate::error::Error;
use crate::models::api::{OptionSession, SelfUserOut};

pub fn config() -> Router<Arc<AppState>> {
	Router::new()
		.nest("/me", Router::new()
			.route("/", get(get_user))
		)
}

pub async fn get_user(
	State(state): State<Arc<AppState>>,
	session: OptionSession,
) -> Result<Json<SelfUserOut>, Error> {
	let session = session.item.ok_or(Error::Unauthorized)?.session;

	let user = services::users::get_user(
		&state,
		session
	).await?;

	Ok(Json(user))
}

