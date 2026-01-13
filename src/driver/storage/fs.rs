use crate::{
    domain::{Storage, StorageId},
    Error, Result,
};
use std::path::{Path, MAIN_SEPARATOR_STR};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

/// Store binary objects in local files.
pub struct FileStorage {
    pub root_dir: String,
}

impl FileStorage {
    /// Create a file storage instance.
    pub fn new(root_dir: String) -> Self {
        Self { root_dir }
    }

    /// Verify that the root dir exists.
    pub fn validate(self) -> Result<Self> {
        if !Path::new(&self.root_dir).exists() {
            return Err(Error::internal(format!("{} doesn't exist", self.root_dir)));
        }
        Ok(self)
    }

    /// Build file-system storage path for a key.
    fn path(&self, file_name: &Uuid) -> String {
        format!("{}{}{}", self.root_dir, MAIN_SEPARATOR_STR, file_name)
    }
}

#[async_trait::async_trait]
impl Storage for FileStorage {
    /// Read bytes from file
    async fn read(&self, StorageId(key): &StorageId) -> Result<Vec<u8>> {
        let bytes = fs::read(self.path(key)).await?;
        Ok(bytes)
    }

    /// Write bytes to file
    async fn write(&self, bytes: &[u8]) -> Result<StorageId> {
        if bytes.is_empty() {
            return Err(Error::invalid_args("empty file"));
        }
        let key = Uuid::new_v4();
        let mut file = File::create(self.path(&key)).await?;
        file.write_all(bytes).await?;
        Ok(StorageId(key))
    }

    /// Delete bytes for a key
    async fn delete(&self, StorageId(key): &StorageId) -> Result<()> {
        fs::remove_file(self.path(key)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fs_storage() {
        // Create a temp root dir
        let temp_dir = std::env::temp_dir().join(format!("fs_storage_test_{}", Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).await.unwrap();

        // Storage type to test
        let storage = FileStorage::new(temp_dir.to_str().unwrap().to_string());

        // Write, read, then delete some binary data.
        let data = b"The quick brown fox jumped over the lazy dog";
        let key = storage.write(data).await.unwrap();
        let read_data = storage.read(&key).await.unwrap();
        assert_eq!(read_data, data);
        storage.delete(&key).await.unwrap();

        // Verify file is deleted
        let result = storage.read(&key).await;
        assert!(result.is_err());

        // Cleanup
        fs::remove_dir_all(&temp_dir).await.unwrap();
    }
}
