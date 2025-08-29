use actix_web::{patch, web::{self, Json}, HttpRequest, HttpResponse, Responder};
use justpic_database::{
    models::{roles::Role, users::DbUser},
    postgres, DatabaseError,
};
use justpic_models::{api::users::UserSelfResponse, Validate};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    auth::extract,
    error::{Error, Result},
};

#[derive(Clone, Deserialize, Validate, ToSchema)]
pub struct ChangeUsernameRequest {
    #[schema(example = "not_john_doe")]
    #[validate(length(min = 3, max = 128))]
    pub username: String,
}

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
    let session =
        extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

	let new_username = payload.into_inner().username;

    let mut user = DbUser::get_by_session(&session, &pool)
        .await?
        .ok_or(Error::Unauthorized)?;
    let user_cache_key = format!("user:{}", user.username);

	user.username = new_username;
    user.update(&pool).await?;

    justpic_cache::remove_from_cache(user_cache_key, &redis_pool).await?;

    let user_out = UserSelfResponse::from(user);
    Ok(HttpResponse::Ok().json(user_out))
}
