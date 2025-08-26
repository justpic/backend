use actix_web::{HttpResponse, get, web};
use justpic_storage::AppStorage;
use tokio_util::io::ReaderStream;

use crate::error::{Error, Result};

/// Get file from s3
#[utoipa::path(get, path = "/v1/picks/{id}/file", tag = "picks")]
#[get("/{id}/file")]
pub async fn get_file(
    s3: web::Data<justpic_storage::S3Storage>,
    id: web::Path<(String,)>,
) -> Result<HttpResponse> {
    let id: &str = id.0.as_ref();
    let storage_key = format!("{}/{}/{}", &id[..2], &id[2..4], &id);

    let file = s3.get(&storage_key).await?.ok_or(Error::NotFound)?;

    let mimetype = file.mimetype.unwrap_or(String::from("bin"));
    let reader = file.body.into_async_read();

    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::Ok().content_type(mimetype).streaming(stream))
}
