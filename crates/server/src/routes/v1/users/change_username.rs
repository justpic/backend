use actix_web::{patch, web::{self, Json}, HttpRequest, HttpResponse, Responder};
use justpic_database::{
    models::{roles::Role, users::DbUser},
    postgres,
};
use justpic_models::api::users::{ChangeUsernameRequest, UserSelfResponse};

use crate::{
    auth::extract,
    error::{Error, Result},
};

/// Change current user name
#[utoipa::path(
    patch, 
    path = "/v1/users/me/username", 
		request_body = ChangeUsernameRequest,
    tag = "users",
    responses(
        (status = 200, body = UserSelfResponse),
        (status = 400)
    )
)]
#[patch("/me/username")]
pub async fn change_username(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
		payload: Json<ChangeUsernameRequest>
) -> Result<impl Responder> {
    // Getting user session from request
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

		let new_username = payload.into_inner().username;

    let mut user = DbUser::get_by_session(&session, &pool)
        .await?
        .ok_or(Error::Unauthorized)?;
		user.username = new_username;

		// Conflict checking

		// Cache invalidation

    // Cleaning up the database model for serving to the Api
    let user_out = UserSelfResponse::from(user);

    Ok(HttpResponse::Ok().json(user_out))
}
