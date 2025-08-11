use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn config() -> Router<Arc<AppState>> {
    Router::new()
}
