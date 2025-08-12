pub mod auth;
pub mod users;

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn config() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::config())
        .nest("/users", users::config())
}
