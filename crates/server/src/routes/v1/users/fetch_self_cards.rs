use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::{cards::Card, roles::Role, users::DbUser},
    postgres,
};
use justpic_models::api::{cards::CardResponse};

use crate::{
    auth::extract,
    error::{Error, Result},
};

/// Get current user cards
#[utoipa::path(
    get, 
    path = "/v1/users/me/cards", 
    tag = "cards",
    responses(
        (status = 200, body = Vec<CardResponse>),
        (status = 400)
    )
)]
#[get("/me/cards")]
pub async fn fetch(
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

	let cards = Card::get_by_owner_id_with_user(&user.id, &pool).await?;

    // Cleaning up the database model for serving to the Api
    let out = cards.into_iter().map(|v| {
			CardResponse::from(v)
		}).collect::<Vec<CardResponse>>();

    Ok(HttpResponse::Ok().json(out))
}
