use crate::Result;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// The newtype storage id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
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
    async fn read(&self, storage_id: &StorageId) -> Result<Vec<u8>>;

    /// Write bytes
    async fn write(&self, bytes: &[u8]) -> Result<StorageId>;

    /// Delete bytes
    async fn delete(&self, storage_id: &StorageId) -> Result<()>;
}
