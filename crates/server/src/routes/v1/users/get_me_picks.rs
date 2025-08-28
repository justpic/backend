use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::{picks::DbPick, roles::Role, users::DbUser},
    postgres,
};
use justpic_models::api::{picks::PickResponse};

use crate::{
    auth::extract,
    error::{Error, Result},
};

/// Get current user picks
#[utoipa::path(
    get, 
    path = "/v1/users/me/picks", 
    tag = "picks",
    responses(
        (status = 200, body = Vec<PickResponse>),
        (status = 400)
    )
)]
#[get("/me/picks")]
pub async fn get_me_picks(
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

	let picks = DbPick::get_by_owner_id_with_user(&user.id, &pool).await?;

    // Cleaning up the database model for serving to the Api
    let out = picks.into_iter().map(|v| {
			PickResponse::from(v)
		}).collect::<Vec<PickResponse>>();

    Ok(HttpResponse::Ok().json(out))
}
