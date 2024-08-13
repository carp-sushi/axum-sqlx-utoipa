#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx_todos::{
    api::{Api, Ctx},
    config::Config,
    driver::message::{email::EmailMessenger, Messenger},
    repo::Repo,
};
use std::{error::Error, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Embed migrations into the server binary.
pub static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars
    dotenv().ok();

    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = Arc::new(Config::default());
    tracing::debug!("Loaded config = {:?}", config);

    // Create pg connection pool
    let pool = config
        .db_pool_opts()
        .connect(config.db_connection_string().as_ref())
        .await?;

    tracing::debug!("Running migrations");
    MIGRATOR.run(&pool).await?;

    // Set up repo
    let pool = Arc::new(pool);
    let repo = Arc::new(Repo::new(pool));

    // Set up message driver
    let messenger = Arc::new(Box::new(EmailMessenger) as Box<dyn Messenger>);

    // Set up API
    let ctx = Ctx::new(repo, messenger);
    let api = Api::new(Arc::new(ctx));

    // Start server
    tracing::info!("Server listening on {}", config.listen_addr);
    let listener = config.tcp_listener().await;
    axum::serve(listener, api.routes()).await?;

    Ok(())
}
