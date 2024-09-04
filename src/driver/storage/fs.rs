use super::Storage;
use crate::{Error, Result};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, MAIN_SEPARATOR_STR},
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
    fn path(&self, file_name: Uuid) -> String {
        format!("{}{}{}", self.root_dir, MAIN_SEPARATOR_STR, file_name)
    }
}

#[async_trait::async_trait]
impl Storage<Uuid> for FileStorage {
    /// Read bytes from file
    async fn read(&self, key: Uuid) -> Result<Vec<u8>> {
        let mut f = File::open(self.path(key))?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Write bytes to file
    async fn write(&self, bytes: &[u8]) -> Result<Uuid> {
        if bytes.is_empty() {
            return Err(Error::invalid_args("empty file"));
        }
        let key = Uuid::new_v4();
        let mut file = File::create(self.path(key))?;
        file.write_all(bytes)?;
        Ok(key)
    }

    /// Delete bytes for a key
    async fn delete(&self, key: Uuid) -> Result<()> {
        fs::remove_file(self.path(key))?;
        Ok(())
    }
}
