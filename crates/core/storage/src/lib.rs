use aws_config::Region;
use aws_sdk_s3::{Client, config::Credentials, error::SdkError, primitives::ByteStream};
use derive_more::{Display, From};

pub type S3Client = Client;

#[derive(Debug, From, Display)]
pub enum StorageError {
    SdkError(String),

    NotFound,

    InvalidState,

    InvalidRequest,

    TooManyParts,
}

pub type StorageResult<T> = Result<T, StorageError>;

/// Initialize S3 connection
pub async fn setup() -> S3Client {
    let access_key = dotenvy::var("S3_ACCESS").expect(".env file does not contain 'S3_ACCESS'");
    let secret_key = dotenvy::var("S3_SECRET").expect(".env file does not contain 'S3_SECRET'");

    let endpoint = dotenvy::var("S3_URI").expect(".env file does not contain 'S3_URI'");

    let creds = Credentials::new(access_key, secret_key, None, None, "minio");
    let region = Region::new("us-east-1");

    let config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(endpoint)
        .credentials_provider(creds)
        .region(region)
        .force_path_style(true)
        .build();

    aws_sdk_s3::Client::from_conf(config)
}

fn get_bucket_from_env() -> String {
    dotenvy::var("S3_BUCKET").unwrap_or(String::from("main"))
}

/// Upload file into S3
pub async fn set_object(
    key: impl AsRef<str>,
    data: Vec<u8>,
    client: &S3Client,
) -> StorageResult<()> {
    match client
        .put_object()
        .bucket(get_bucket_from_env())
        .key(key.as_ref())
        .body(data.into())
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(SdkError::ServiceError(e)) if e.err().is_invalid_request() => {
            Err(StorageError::InvalidRequest)
        }
        Err(SdkError::ServiceError(e)) if e.err().is_too_many_parts() => {
            Err(StorageError::TooManyParts)
        }
        Err(e) => Err(StorageError::SdkError(format!("PUT ERROR: {e:?}"))),
    }
}

/// Get file from S3
pub async fn get_object(key: impl AsRef<str>, client: &S3Client) -> StorageResult<ByteStream> {
    let res = client
        .get_object()
        .bucket(get_bucket_from_env())
        .key(key.as_ref())
        .send()
        .await;

    match res {
        Ok(res) => Ok(res.body),
        Err(SdkError::ServiceError(e)) if e.err().is_no_such_key() => Err(StorageError::NotFound),
        Err(SdkError::ServiceError(e)) if e.err().is_invalid_object_state() => {
            Err(StorageError::InvalidState)
        }
        Err(e) => Err(StorageError::SdkError(format!("GET ERROR: {e:?}"))),
    }
}

/// Delete file from S3
pub async fn del_object(key: impl AsRef<str>, client: &S3Client) {
    //
}
