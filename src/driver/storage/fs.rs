use super::Storage;
use crate::{Error, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::MAIN_SEPARATOR_STR,
};
use uuid::Uuid;

/// Store binary objects in local files.
pub struct FileStorage {
    pub root: String,
}

impl FileStorage {
    /// Create a file storage instance.
    pub fn new(root: String) -> Self {
        Self { root }
    }

    fn path(&self, key: Uuid) -> String {
        format!("{}{}{}", self.root, MAIN_SEPARATOR_STR, key)
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
}
