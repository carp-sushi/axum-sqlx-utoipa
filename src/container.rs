use crate::{
    config::Config,
    domain::StorageId,
    driver::storage::{fs::FileStorage, Storage},
    keeper::{
        FileKeeper, FileKeeperPostgres, StoryKeeper, StoryKeeperPostgres, TaskKeeper,
        TaskKeeperPostgres,
    },
    repo::Repo,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

/// A simple dependency container.
pub struct Container {
    pub config: Arc<Config>,
}

impl Container {
    /// Create new container
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Load storage system.
    pub fn storage(&self) -> impl Storage<StorageId> {
        assert!(self.config.storage_type == "file"); // TODO: support gcs, s3, etc...
        FileStorage::new(self.config.storage_bucket.clone())
            .validate()
            .expect("unable to load file storage")
    }

    /// Load database connection pool.
    pub async fn pg_pool(&self) -> Pool<Postgres> {
        let url = self.config.db_connection_string();
        self.config
            .db_pool_opts()
            .connect(url.as_ref())
            .await
            .expect("unable to load pg pool")
    }

    /// Load database logic
    pub fn repo(&self, pool: Pool<Postgres>) -> Repo {
        Repo::new(Arc::new(pool))
    }

    /// Load story persistence api
    pub fn story_keeper(&self, repo: Arc<Repo>) -> impl StoryKeeper {
        StoryKeeperPostgres::new(repo)
    }

    /// Load task persistence api
    pub fn task_keeper(&self, repo: Arc<Repo>) -> impl TaskKeeper {
        TaskKeeperPostgres::new(repo)
    }

    /// Load file metadata persistence api
    pub fn file_keeper(&self, repo: Arc<Repo>) -> impl FileKeeper {
        FileKeeperPostgres::new(repo)
    }
}
