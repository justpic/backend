use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, post, web};

use futures_util::{StreamExt, TryStreamExt};
use justpic_database::models::picks::DbPick;
use justpic_database::models::roles::Role;
use justpic_database::postgres;

use justpic_models::api::picks::UploadDto;
use justpic_storage::temp::TempFile;
use justpic_storage::{AppStorage, S3Stream};

use crate::auth::extract;
use crate::error::{Error, Result};


/// Create new "pick" endpoint
#[utoipa::path(
	post, 
	path = "/v1/picks/", 
	tag = "picks",
	responses(
        (status = 201, description = "Pick created"),
    )
)]
#[post("/")]
pub async fn create(
    req: HttpRequest,
    mut payload: Multipart,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
    s3: web::Data<justpic_storage::S3Storage>,
) -> Result<HttpResponse> {
		let session =
			extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

		let user_id = session.user_id;
		drop(session);

		let mut pick = DbPick::new(
			None, None, 
			None, user_id, 
			"bin", true, 
			false, false
		);
		let pick_id = &pick.id;
		let mut is_file_appended = false;

		while let Some(Ok(mut field)) = payload.next().await {				
				let mime = field.content_type().ok_or(Error::BadRequest)?.to_string();

				match field.name() {
						Some("file") => {
							let mapped_field = field
								.map_err(|e| std::io::Error::other(format!("{e:?}")));
							let mut temp = TempFile::from_stream(mapped_field)
								.with_id(pick_id.to_string());

							let temp_path = temp.save().await?;
							
							let s3_stream = S3Stream::from_path(temp_path)
								.await
								.map_err(|e| Error::StorageError(justpic_storage::StorageError::SdkError(format!("{e:?}"))))?;
							
							let key = temp.get_storage_key()?;
							s3.upload(&key, s3_stream).await?;
							
							temp.remove().await?;
							
							is_file_appended = true;
							pick.mimetype = mime;
						},
						Some("meta") => {
							let meta_raw = field.bytes(5 * 1024).await.map_err(|_| Error::BadRequest)??.to_vec();
							let meta = UploadDto::from_slice(&meta_raw).ok_or(Error::BadRequest)?;

							pick.title = meta.title;
							pick.description = meta.description;
							pick.source_url = meta.source;
							pick.private = meta.private;
							pick.ai_generated = meta.ai_generated;
						},
						_ => continue,
				};
		}

		if !is_file_appended {
			return Err(Error::BadRequest);
		}

		pick.insert(&pool).await?;

    Ok(HttpResponse::Created().json(pick))
}
