use std::str::FromStr;

use actix_web::{delete, web, HttpRequest, HttpResponse, Responder};
use justpic_database::{
    models::{cards::Card, roles::Role},
    postgres,
};
use uuid::Uuid;

use crate::{
    auth::extract, error::{Error, Result}
};

/// Delete card by id
#[utoipa::path(
    delete, 
    path = "/v1/cards/{id}", 
    tag = "cards",
    responses(
        (status = 204),
        (status = 400)
    )
)]
#[delete("/{id}")]
pub async fn delete_card(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    id: web::Path<String>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
		let card_id = Uuid::from_str(&id).map_err(|_| Error::BadRequest)?;
		
		let session = extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;
		let user_id = session.user_id;

		let card = Card::get_by_id(&card_id, &pool).await?
			.ok_or(Error::NotFound)?;
		let card_owner_id = card.owner_id.ok_or(Error::Forbidden)?;

		if card_owner_id != user_id {
			return Err(Error::Forbidden);
		}

		card.remove(&pool).await?;

    Ok(HttpResponse::NoContent().finish())
}
