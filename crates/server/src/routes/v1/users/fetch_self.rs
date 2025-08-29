use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::{roles::Role, users::DbUser},
    postgres,
};
use justpic_models::api::users::UserSelfResponse;

use crate::{
    auth::extract,
    error::{Error, Result},
};

/// Get current user by session
#[utoipa::path(
    get, 
    path = "/v1/users/me", 
    tag = "users",
    responses(
        (status = 200, body = UserSelfResponse),
        (status = 400)
    )
)]
#[get("/me")]
pub async fn fetch_self(
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

    // Cleaning up the database model for serving to the Api
    let user_out = UserSelfResponse::from(user);

    Ok(HttpResponse::Ok().json(user_out))
}
