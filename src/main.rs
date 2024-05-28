#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx_todos::{
    api::{Api, Ctx},
    config::Config,
};
use std::{env, error::Error, sync::Arc};

// Embed migrations into the server binary.
pub static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load env vars
    dotenv().ok();

    // Init tracing
    if env::var("DEV_MODE").is_ok() {
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    // Load config
    let config = Arc::new(Config::default());
    tracing::debug!("Loaded config = {:?}", config);

    // Create pg connection pool
    let pool = config
        .db_pool_opts()
        .connect(config.db_connection_string().as_ref())
        .await?;

    tracing::info!("Running migrations");
    MIGRATOR.run(&pool).await?;

    // Set up API
    let ctx = Ctx::new(Arc::clone(&config), Arc::new(pool));
    let api = Api::new(Arc::new(ctx));

    // Start server
    tracing::info!("Server listening on {}", config.listen_addr);
    axum::serve(config.tcp_listener(), api.routes()).await?;

    Ok(())
}
