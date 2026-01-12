use crate::config::Config;
use crate::domain::StorageId;
use crate::driver::storage::{fs::FileStorage, mem::MemoryStorage, Storage};

impl Config {
    /// Load a dynamic storage instance.
    pub fn load_storage(&self) -> Box<dyn Storage<StorageId>> {
        match self.storage_type.as_str() {
            "file" => Box::new(FileStorage::new(self.storage_bucket.clone())),
            _ => Box::new(MemoryStorage::new()),
        }
    }
}
