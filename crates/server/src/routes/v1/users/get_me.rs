use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::{roles::Role, users::DbUser},
    postgres, redis,
};
use justpic_models::api::users::UserSelfOut;

use crate::{
    auth::extract,
    error::{Error, Result},
};

/// Login endpoint
#[utoipa::path(get, path = "/v1/users/me", tag = "users")]
#[get("/me")]
pub async fn get_me(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<redis::Pool>,
) -> Result<impl Responder> {
    // Getting user session from request
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

    // todo: change db model to public api model
    let user = DbUser::get_by_session(&session, &pool)
        .await?
        .ok_or(Error::Unauthorized)?;

    let user_out = UserSelfOut::from(user);

    Ok(HttpResponse::Ok().json(user_out))
}
