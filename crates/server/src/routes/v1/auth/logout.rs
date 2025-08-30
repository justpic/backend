use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use justpic_database::{models::roles::Role, postgres};

use crate::{
    auth::{extract, generate_session_cache_key, generate_session_cookie},
    error::Result,
};

/// Logout endpoint
#[utoipa::path(post, path = "/v1/auth/logout", tag = "auth")]
#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

    session.remove(&pool).await?;
    justpic_cache::remove_from_cache(
        generate_session_cache_key(&session.session_key),
        &redis_pool,
    )
    .await?;

    let mut cookie = generate_session_cookie(&session);
    cookie.make_removal();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}
