use crate::{
    config::Config,
    domain::Storage,
    driver::storage::{fs::FileStorage, mem::MemoryStorage, minio::MinioStorage},
};

use minio::s3::Client;
use minio::s3::{creds::StaticProvider, http::BaseUrl};

impl Config {
    /// Load a dynamic storage instance.
    pub fn load_storage(&self) -> Box<dyn Storage> {
        match self.storage_type.as_str() {
            "file" => Box::new(FileStorage::new(self.storage_bucket.clone())),
            "minio" => Box::new(MinioStorage::new(self)),
            _ => Box::new(MemoryStorage::new()),
        }
    }

    /// Create a MinIO client from this config. WARN: panics on misconfiguration.
    pub fn create_minio_client(&self) -> Client {
        let access_key = self
            .storage_minio_access_key
            .clone()
            .expect("minio access key not set");

        let secret_key = self
            .storage_minio_secret_key
            .clone()
            .expect("minio secret key not set");

        let provider = StaticProvider::new(&access_key, &secret_key, None);

        let base_url: BaseUrl = self
            .storage_minio_base_url
            .clone()
            .expect("minio base url not set")
            .parse()
            .expect("unable to parse minio base URL");

        Client::new(base_url, Some(Box::new(provider)), None, None)
            .expect("unable to create minio client")
    }
}
