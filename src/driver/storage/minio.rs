use crate::{
    config::Config,
    domain::{Storage, StorageId},
    Error, Result,
};

use bytes::Bytes;
use minio::s3::{
    error::Error as MinioError, segmented_bytes::SegmentedBytes, types::S3Api, Client,
};
use uuid::Uuid;

/// MinIO object storage driver.
pub struct MinioStorage {
    bucket: String,
    client: Client,
}

impl MinioStorage {
    /// Create a new MinIO storage instance.
    pub fn new(config: &Config) -> Self {
        Self {
            bucket: config.storage_bucket.clone(),
            client: config.create_minio_client(),
        }
    }
}

#[async_trait::async_trait]
impl Storage for MinioStorage {
    /// Read object
    async fn read(&self, StorageId(uuid): &StorageId) -> Result<Bytes> {
        let get_object = self
            .client
            .get_object(&self.bucket, uuid.to_string())
            .send()
            .await?;
        let segmented_bytes = get_object.content.to_segmented_bytes().await?;
        Ok(segmented_bytes.to_bytes())
    }

    /// Write object
    async fn write(&self, bytes: Bytes) -> Result<StorageId> {
        let uuid = Uuid::new_v4();
        self.client
            .put_object(&self.bucket, uuid.to_string(), SegmentedBytes::from(bytes))
            .send()
            .await?;
        Ok(StorageId(uuid))
    }

    /// Delete object
    async fn delete(&self, &StorageId(uuid): &StorageId) -> Result<()> {
        self.client
            .delete_object(&self.bucket, uuid.to_string())
            .send()
            .await?;
        Ok(())
    }
}

// Map MinIO errors as internal errors for this project.
impl From<MinioError> for Error {
    fn from(err: MinioError) -> Self {
        Error::internal(err.to_string())
    }
}
