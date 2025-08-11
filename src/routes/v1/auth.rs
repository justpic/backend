use std::sync::Arc;
use axum::extract::State;
use axum::Router;
use crate::AppState;
use crate::error::Error;
use crate::models::api::SelfUserOut;

pub fn config() -> Router<Arc<AppState>> {
	Router::new()
}

pub async fn register(
	State(state): State<Arc<AppState>>
) -> Result<SelfUserOut, Error> {
	todo!()
}