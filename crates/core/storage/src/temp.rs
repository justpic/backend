use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::StreamExt;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

const TEMP_DIR: &str = "./tmp";

#[derive(Debug)]
pub struct TempFile<T>
where
    T: StreamExt<Item = Result<Bytes, std::io::Error>> + Unpin,
{
    id: Option<String>,
    stream: T,
    path: Option<PathBuf>,
}

impl<T> TempFile<T>
where
    T: StreamExt<Item = Result<Bytes, std::io::Error>> + Unpin,
{
    pub fn from_stream(stream: T) -> Self {
        TempFile {
            id: None,
            stream,
            path: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn get_storage_key(&self) -> Result<String, std::io::Error> {
        let id = self
            .id
            .clone()
            .ok_or(std::io::Error::other("TempFile was not initialized"))?;
        Ok(format!("{}/{}/{}", &id[0..2], &id[2..4], &id))
    }

    pub async fn save(&mut self) -> Result<PathBuf, std::io::Error> {
        let tmp_dir = Path::new(TEMP_DIR);
        tokio::fs::create_dir_all(tmp_dir).await?;

        let id = self
            .id
            .clone()
            .ok_or(std::io::Error::other("TempFile was not initialized"))?;

        let path_str = tmp_dir.join(id);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(Path::new(&path_str))
            .await?;

        while let Some(chunk) = self.stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        file.flush().await?;

        let path = PathBuf::from(&path_str);
        self.path = Some(path.clone());
        Ok(path)
    }

    pub async fn remove(self) -> Result<(), std::io::Error> {
        let path = self
            .path
            .clone()
            .ok_or(std::io::Error::other("TempFile was not created yet"))?;

        tokio::fs::remove_file(path).await?;
        drop(self);
        Ok(())
    }
}
