use std::str::FromStr;

use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use justpic_database::{
    models::cards::Card,
    postgres,
};
use justpic_models::api::cards::CardResponse;
use uuid::Uuid;

use crate::{
    auth::extract, error::{Error, Result}
};

/// Get card by id
#[utoipa::path(
    get, 
    path = "/v1/card/{id}", 
    tag = "cards",
    responses(
        (status = 200, body = CardResponse),
        (status = 400)
    )
)]
#[get("/{id}")]
pub async fn fetch(
    req: HttpRequest,
    pool: web::Data<postgres::Pool>,
    id: web::Path<String>,
    redis_pool: web::Data<justpic_cache::Pool>,
) -> Result<impl Responder> {
	let id = Uuid::from_str(&id).map_err(|_| Error::BadRequest)?;

    let out = justpic_cache::cache_wrapper::<CardResponse, Error, _, _>(
        &redis_pool, 
        format!("card:{id}"),
        async move || {
            let card = Card::get_by_id_with_user(&id, &pool).await?
                .ok_or(Error::NotFound)?;

            let out = CardResponse::from(card);
            Ok(out)
        }
    ).await?;

    if let Some(owner) = &out.owner {
        if out.private {
            let session = extract::get_maybe_session_from_request(&req, &redis_pool).await?
                .ok_or(Error::Forbidden)?;
            
            if owner.id != session.user_id {
                return Err(Error::Forbidden);
            }
        }
    }

    Ok(HttpResponse::Ok().json(out))
}
