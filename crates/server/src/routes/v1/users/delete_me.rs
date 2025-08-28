use actix_web::{delete, web, HttpRequest, HttpResponse, Responder};
use justpic_database::{
    models::{roles::Role, sessions::DbSession, users::DbUser},
    postgres,
};

use crate::{
    auth::{extract, generate_session_cache_key},
    error::{Error, Result},
};

/// Delete current user
#[utoipa::path(
    delete, 
    path = "/v1/users/me", 
    tag = "users",
    responses(
        (status = 204, description = "Account successfully deleted"),
        (status = 400, description = "The current session does not contain a user")
    )
)]
#[delete("/me")]
pub async fn delete_me(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
    // Getting user session from request
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

    let user = DbUser::get_by_session(&session, &pool)
        .await?
        .ok_or(Error::Unauthorized)?;
    
    let user_cache_key = format!("user:{}", user.username);
    justpic_cache::remove_from_cache(user_cache_key, &redis_pool).await?;
    
    let user_sessions = DbSession::get_by_owner_id(user.id, &pool).await?;
    for session in user_sessions {
        justpic_cache::remove_from_cache(
            generate_session_cache_key(&session.session_key), 
            &redis_pool
        ).await?;
    }
    
    user.remove(&pool).await?;
    
    Ok(HttpResponse::NoContent().finish())
}
