use std::env;

mod database;
mod storage;
mod tcp;

/// Configuration settings
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub db_max_connections: u32,
    pub db_url: String,
    pub db_schema: String,
    pub storage_type: String,
    pub storage_bucket: String,
}

/// Default for config just calls basic constructor
impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        // http server settings
        let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".into());
        let listen_addr = format!("0.0.0.0:{port}");

        // database settings
        let mut db_max_connections = num_cpus::get() as u32;
        if let Ok(s) = env::var("DATABASE_MAX_CONNECTIONS") {
            db_max_connections = s.parse().expect("DB_MAX_CONNECTIONS could not be parsed")
        }
        let db_url = env::var("DATABASE_URL").expect("DB_HOST not set");
        let db_schema = env::var("DATABASE_SCHEMA").unwrap_or("public".to_string());
        let storage_type = env::var("STORAGE_TYPE").expect("STORAGE_TYPE not set");
        let storage_bucket = env::var("STORAGE_BUCKET").unwrap_or(".storage".to_string());

        // Create config
        Self {
            listen_addr,
            db_max_connections,
            db_url,
            db_schema,
            storage_type,
            storage_bucket,
        }
    }
}
