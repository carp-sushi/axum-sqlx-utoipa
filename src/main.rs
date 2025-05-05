#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx_todos::{
    api::{Api, Ctx},
    config::Config,
    driver::storage::{fs::FileStorage, mem::MemoryStorage, Storage},
    repo::Repo,
};
use std::{error::Error, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// Embed migrations into the server binary.
pub static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars and tracing subscriber
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = Arc::new(Config::default());

    // Load connection pool and run schema migrations
    let pool = config.db_pool_opts().connect(&config.db_url).await?;
    tracing::debug!("Running migrations");
    MIGRATOR.run(&pool).await?;

    // Set up storage and repo

    let storage: Box<dyn Storage<Uuid>> = if &config.storage_type == "file" {
        tracing::debug!("Using file storage");
        Box::new(FileStorage::new(config.storage_bucket.clone()))
    } else {
        tracing::debug!("Using memory storage");
        Box::new(MemoryStorage::new())
    };
    let repo = Arc::new(Repo::new(Arc::new(pool)));

    // Set up API
    let ctx = Ctx::new(Arc::new(storage), repo);
    let service = Api::new(Arc::new(ctx)).mk_service();

    // Start server
    tracing::info!("Server listening on {}", config.listen_addr);
    let listener = config.tcp_listener().await;
    axum::serve(listener, service).await?;

    Ok(())
}
