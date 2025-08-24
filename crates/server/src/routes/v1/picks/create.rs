use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::{MultipartForm, json::Json as MpJson};
use actix_web::{HttpRequest, HttpResponse, post, web};

use utoipa::ToSchema;
use uuid::{Uuid};

use justpic_database::models::picks::DbPick;
use justpic_database::models::roles::Role;
use justpic_database::postgres;

use justpic_models::api::picks::UploadDto;

use justpic_storage::{AppStorage, S3Stream};

use crate::auth::extract;
use crate::error::{Error, Result};

/// Upload 'pick' multipart form
#[derive(Debug, MultipartForm, ToSchema)]
struct UploadForm {
	#[schema(value_type = String, format = Binary)]
	#[multipart(limit = "35MB")]
	file: TempFile,

	#[schema(value_type = UploadDto)]
	meta: MpJson<UploadDto>,
}

/// Create new "pick" endpoint
#[utoipa::path(
	post, 
	path = "/v1/picks/", 
	request_body (
		content = UploadForm,
		content_type = "multipart/form-data"
	),
	tag = "picks",
	responses(
        (status = 201, description = "Pick created"),
    )
)]
#[post("/")]
pub async fn create(
    req: HttpRequest,
    MultipartForm(payload): MultipartForm<UploadForm>,
    pool: web::Data<postgres::Pool>,
    redis_pool: web::Data<justpic_cache::Pool>,
    s3: web::Data<justpic_storage::S3Storage>,
) -> Result<HttpResponse> {
		// TODO: Add logging
		let session =
			extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;

		let user_id = session.user_id;

		let id = Uuid::new_v4();
		let str_id = id.to_string();

		let file = payload.file;
		let key = format!("{}/{}/{}", &str_id[0..2], &str_id[2..4], &str_id);

		// TODO: Optimize file uploading
		let s3_stream = S3Stream::from_path(file.file.path())
			.await
			.map_err(|e| Error::StorageError(justpic_storage::StorageError::SdkError(format!("{e:?}"))))?;

		s3.upload(&key, s3_stream).await?;

		let mimetype = file.content_type
			.map(|v| v.to_string())
			.ok_or(Error::BadRequest)?;

		let meta = payload.meta.into_inner();

		let new_pick = DbPick::new(
			id, meta.title, 
			meta.description, 
			meta.source, user_id, 
			mimetype,	meta.private,
			meta.ai_generated, meta.nsfw
		);

		new_pick.insert(&pool).await?;

    Ok(HttpResponse::Created().json(new_pick))
}
