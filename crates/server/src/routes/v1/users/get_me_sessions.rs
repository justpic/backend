use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::{roles::Role, sessions::DbSession},
    postgres,
};
use justpic_models::api::auth::SessionOut;

use crate::{auth::extract, error::Result};

/// Get current user sessions list by session
#[utoipa::path(get, path = "/v1/users/me/sessions", tag = "users")]
#[get("/me/sessions")]
pub async fn get_me_sessions(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

    let user_id = session.user_id;

    let sessions = DbSession::get_by_owner_id(user_id, &pool)
        .await?
        .into_iter()
        .map(SessionOut::from)
        .collect::<Vec<SessionOut>>();

    Ok(HttpResponse::Ok().json(sessions))
}
