use super::Storage;
use crate::{domain::StorageId, Error, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, MAIN_SEPARATOR_STR},
};

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
            return Err(Error::internal(format!("{} does not exist", self.root_dir)));
        }
        Ok(self)
    }

    /// Build file-system storage path for a key.
    fn path(&self, StorageId(file_name): StorageId) -> String {
        format!("{}{}{}", self.root_dir, MAIN_SEPARATOR_STR, file_name)
    }
}

#[async_trait::async_trait]
impl Storage<StorageId> for FileStorage {
    /// Read bytes from file
    async fn read(&self, key: StorageId) -> Result<Vec<u8>> {
        let mut f = File::open(self.path(key))?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Write bytes to file
    async fn write(&self, bytes: &[u8]) -> Result<StorageId> {
        if bytes.is_empty() {
            return Err(Error::invalid_args("empty file"));
        }
        let key = StorageId::random();
        let mut file = File::create(self.path(key.clone()))?;
        file.write_all(bytes)?;
        Ok(key)
    }

    /// Delete bytes for a key
    async fn delete(&self, key: StorageId) -> Result<()> {
        std::fs::remove_file(self.path(key))?;
        Ok(())
    }
}
