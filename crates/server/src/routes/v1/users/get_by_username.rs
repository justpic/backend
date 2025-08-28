use actix_web::{HttpResponse, Responder, get, web};

use justpic_database::{models::users::DbUser, postgres};
use justpic_models::api::users::UserResponse;

use crate::error::{Error, Result};

/// Get user by username
#[utoipa::path(
    get, 
    path = "/v1/users/name/{username}", 
    tag = "users",
    responses(
        (status = 200, body = UserResponse),
        (status = 404)
    )
)]
#[get("/name/{username}")]
pub async fn get_by_username(
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
    username: web::Path<(String,)>,
) -> Result<impl Responder> {
    let name = username.into_inner().0;
    let key = format!("user:{name}");

    let user = justpic_cache::cache_wrapper::<UserResponse, Error, _, _>(
        &redis_pool,
        key,
        move || async {
            let pool = pool;
            let user = DbUser::get_by_username(name, &pool)
                .await?
                .ok_or(Error::NotFound)?;

            let out = UserResponse::from(user);

            Ok(out)
        },
    )
    .await?;

    Ok(HttpResponse::Ok().json(user))
}
