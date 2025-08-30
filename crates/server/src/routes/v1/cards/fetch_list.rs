use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use justpic_database::{
    models::cards::Card,
    postgres,
};
use justpic_models::api::cards::CardResponse;

use crate::{
    auth::extract, error::{Result}
};

/// Get cards list
#[utoipa::path(
    get, 
    path = "/v1/cards/", 
    tag = "cards",
    responses(
        (status = 200, body = Vec<CardResponse>),
        (status = 400)
    )
)]
#[get("/")]
pub async fn fetch_list(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
    let session = extract::get_maybe_session_from_request(&req, &redis_pool).await?;
		let user_id = match session {
				Some(s) => Some(s.user_id),
				None => None,
		};

		let list = Card::get_many_with_user(&pool, user_id, 50, 0).await?;
		let out = list.into_iter()
			.map(|v| {
				CardResponse::from(v)
			})
			.collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(out))
}
