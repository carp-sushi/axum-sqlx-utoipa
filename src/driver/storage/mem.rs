use super::Storage;
use crate::{Error, Result};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory binary object storage.
/// NOTE: This implementation of `Storage` panics, so it is only useful for testing.
type DataStore = Arc<RwLock<HashMap<Uuid, Vec<u8>>>>;

/// Store binary objects in memory.
#[derive(Default)]
pub struct MemoryStorage {
    pub datastore: DataStore,
}

impl MemoryStorage {
    /// Create a new memory storage instance.
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait::async_trait]
impl Storage<Uuid> for MemoryStorage {
    /// Read object for a key
    async fn read(&self, key: Uuid) -> Result<Vec<u8>> {
        if let Ok(map) = self.datastore.read() {
            if let Some(value) = map.get(&key) {
                return Ok(value.clone());
            }
        }
        Err(Error::not_found("file not found".into()))
    }

    /// Write object to datastore and return an lookup key.
    async fn write(&self, bytes: &[u8]) -> Result<Uuid> {
        if bytes.is_empty() {
            return Err(Error::invalid_args("empty file"));
        }
        let key = Uuid::new_v4();
        if let Ok(mut map) = self.datastore.write() {
            map.insert(key, bytes.to_vec());
        }
        Ok(key)
    }

    /// Delete object for a key
    async fn delete(&self, key: Uuid) -> Result<()> {
        if let Ok(mut map) = self.datastore.write() {
            map.remove(&key);
        }
        Ok(())
    }
}
