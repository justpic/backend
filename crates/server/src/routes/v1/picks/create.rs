use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::{MultipartForm, json::Json as MpJson};
use actix_web::{HttpRequest, HttpResponse, post, web};

use tracing::error;
use utoipa::ToSchema;
use uuid::{Uuid};

use justpic_database::models::picks::{DbPick, Status};
use justpic_database::models::roles::Role;
use justpic_database::postgres;

use justpic_models::api::picks::UploadRequest;

use justpic_storage::{AppStorage, S3Stream};

use crate::auth::extract;
use crate::error::{Error, Result};

/// Upload 'pick' multipart form
#[derive(Debug, MultipartForm, ToSchema)]
struct UploadForm {
	#[schema(value_type = String, format = Binary)]
	#[multipart(limit = "45MB")]
	file: TempFile,

	#[schema(value_type = UploadRequest)]
	meta: MpJson<UploadRequest>,
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
		let session =
			extract::get_session_from_request(&req, Role::Regular, &pool, &redis_pool).await?;
		let user_id = session.user_id;

		let file = payload.file;
		let mimetype = file.content_type
			.clone()
			.map(|v| v.to_string())
			.ok_or(Error::BadRequest)?;

		let meta = payload.meta.into_inner();

		let id = Uuid::new_v4();
		let new_pick = Arc::new(DbPick::new(
			id, meta.title, 
			meta.description, 
			meta.source, user_id, 
			mimetype,	meta.private,
			meta.ai_generated, meta.nsfw
		));

		new_pick.insert(&pool).await?;

		let s3 = s3.into_inner();
		let pool = pool.into_inner();
		let pick = Arc::clone(&new_pick);

		tokio::spawn(async move {
			process_file_upload(&s3, &pool, &pick, file).await.ok();
		});

    Ok(HttpResponse::Created().json(new_pick))
}

/// Generate the file key and put it into S3
async fn upload_file(
	s3: &justpic_storage::S3Storage,
	mimetype: &str,
	file: TempFile,
	id: String,
) -> std::result::Result<String, justpic_storage::StorageError> {
	let content_type = file.content_type;
	let ext = content_type
		.map(|v| v.subtype().to_string())
		.unwrap_or(String::from("bin"));

	let key = format!("{}/{}/{}.{}", &id[0..2], &id[2..4], &id, ext);

	let stream = S3Stream::from_path(file.file.path())
			.await
			.map_err(|e| justpic_storage::StorageError::SdkError(format!("{e:?}")))?;

	s3.upload(&key, stream, mimetype).await?;

	Ok(format!("{}.{}", id, ext))
}

/// Set 'pick' status and log error
async fn set_status_with_log(
	pick: &DbPick,
	status: Status,
	pool: &postgres::Pool,
) -> Result<()> {
	pick.set_status(status.clone(), pool).await.inspect_err(|e| {
		error!(
			"Failed to set status {:?} for pick {:?}: {e:?}",
			status, pick.id
		);
	})?;

	Ok(())
}

/// Process the file and change the 'pick' status
async fn process_file_upload(
	s3: &justpic_storage::S3Storage,
	pool: &postgres::Pool,
	pick: &DbPick,
	file: TempFile,
) -> Result<()> {
	set_status_with_log(pick, Status::Processing, pool).await?;

	match upload_file(s3, &pick.mimetype, file, pick.id.to_string()).await {
			Ok(key) => {
				pick.set_file_url(key, pool).await?;
				set_status_with_log(pick, Status::Ready, pool).await?;
			},
			Err(e) => {
				error!("Failed to upload file into s3: {e:?}");
				set_status_with_log(pick, Status::Failed, pool).await?;

				return Err(Error::InternalError);
			}
	}
	Ok(())
}