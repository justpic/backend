use aws_config::Region;
use aws_sdk_s3::{Client, config::Credentials, error::SdkError, primitives::ByteStream};
use derive_more::{Display, From};

#[derive(Debug, From, Display)]
pub enum StorageError {
    NotFound,
    InvalidState,
    InvalidRequest,
    TooManyParts,
    SdkError(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

pub type S3Stream = ByteStream;

pub mod temp;

#[derive(Debug, Clone)]
pub struct S3Storage {
    client: Client,
    bucket_name: String,
}

/// Initialize S3 connection
pub async fn setup_s3() -> S3Storage {
    let access_key = dotenvy::var("S3_ACCESS").expect(".env file does not contain 'S3_ACCESS'");
    let secret_key = dotenvy::var("S3_SECRET").expect(".env file does not contain 'S3_SECRET'");
    let bucket_name = dotenvy::var("S3_BUCKET").unwrap_or(String::from("main"));

    let endpoint = dotenvy::var("S3_URI").expect(".env file does not contain 'S3_URI'");

    let creds = Credentials::new(access_key, secret_key, None, None, "minio");
    let region = Region::new("us-east-1");

    let config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(endpoint)
        .credentials_provider(creds)
        .region(region)
        .force_path_style(true)
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);

    S3Storage {
        client,
        bucket_name,
    }
}

#[async_trait::async_trait]
pub trait AppStorage {
    /// Upload file into S3
    async fn upload(&self, key: &str, data: ByteStream) -> StorageResult<()>;

    /// Get file from S3
    async fn get(&self, key: &str) -> StorageResult<ByteStream>;
}

#[async_trait::async_trait]
impl AppStorage for S3Storage {
    async fn upload(&self, key: &str, data: ByteStream) -> StorageResult<()> {
        match self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(data)
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

    async fn get(&self, key: &str) -> StorageResult<ByteStream> {
        match self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
        {
            Ok(res) => Ok(res.body),
            Err(SdkError::ServiceError(e)) if e.err().is_no_such_key() => {
                Err(StorageError::NotFound)
            }
            Err(SdkError::ServiceError(e)) if e.err().is_invalid_object_state() => {
                Err(StorageError::InvalidState)
            }
            Err(e) => Err(StorageError::SdkError(format!("GET ERROR: {e:?}"))),
        }
    }
}
