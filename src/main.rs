#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx_todos::{
    api::{Api, Ctx},
    config::Config,
    container::Container,
    domain::StorageId,
    driver::storage::Storage,
    keeper::{FileKeeper, StoryKeeper, TaskKeeper},
};
use std::{error::Error, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Load config and dependency container
    let config = Arc::new(Config::default());
    let container = Container::new(config.clone());

    // Load connection pool and run schema migrations
    let pool = container.pg_pool().await;
    tracing::debug!("Running migrations");
    MIGRATOR.run(&pool).await?;

    // Set up storage
    let storage = Box::new(container.storage()) as Box<dyn Storage<StorageId>>;

    // Set up persistence APIs
    let repo = Arc::new(container.repo(pool));
    let story_keeper = Box::new(container.story_keeper(repo.clone())) as Box<dyn StoryKeeper>;
    let task_keeper = Box::new(container.task_keeper(repo.clone())) as Box<dyn TaskKeeper>;
    let file_keeper = Box::new(container.file_keeper(repo.clone())) as Box<dyn FileKeeper>;

    // Set up API
    let ctx = Ctx::new(
        Arc::new(storage),
        Arc::new(story_keeper),
        Arc::new(task_keeper),
        Arc::new(file_keeper),
    );
    let service = Api::new(Arc::new(ctx)).mk_service();

    // Start server
    tracing::info!("Server listening on {}", config.listen_addr);
    let listener = config.tcp_listener().await;
    axum::serve(listener, service).await?;

    Ok(())
}
