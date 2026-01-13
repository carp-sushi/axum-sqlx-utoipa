use crate::config::Config;
use crate::domain::Storage;
use crate::driver::storage::{fs::FileStorage, mem::MemoryStorage};

impl Config {
    /// Load a dynamic storage instance.
    pub fn load_storage(&self) -> Box<dyn Storage> {
        match self.storage_type.as_str() {
            "file" => Box::new(FileStorage::new(self.storage_bucket.clone())),
            _ => Box::new(MemoryStorage::new()),
        }
    }
}
