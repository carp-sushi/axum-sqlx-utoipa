use crate::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// The newtype storage id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, ToSchema)]
pub struct StorageId(pub Uuid);

// Display the inner uuid.
impl std::fmt::Display for StorageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Read, write, and delete binary objects.
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Read bytes
    async fn read(&self, storage_id: &StorageId) -> Result<Bytes>;

    /// Write bytes
    async fn write(&self, bytes: Bytes) -> Result<StorageId>;

    /// Delete bytes
    async fn delete(&self, storage_id: &StorageId) -> Result<()>;
}
