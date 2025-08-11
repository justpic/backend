pub mod v1;

use axum::Router;
use std::sync::Arc;

use crate::AppState;

pub fn config() -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::config())
}
