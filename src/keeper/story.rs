use crate::{domain::Story, repo::Repo, Result};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// Story persistence API
#[async_trait::async_trait]
pub trait StoryKeeper: Send + Sync {
    /// Fetch a story by ID
    async fn fetch(&self, id: Uuid) -> Result<Story>;

    /// Get a page of stories
    async fn list(&self, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)>;

    /// Create a new story
    async fn create(&self, name: String) -> Result<Story>;

    /// Update an existing story's name
    async fn update(&self, id: Uuid, name: String) -> Result<Story>;

    /// Delete a story and all child files and tasks
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Concrete persistence API for stories using postgres
pub struct StoryKeeperPostgres {
    repo: Arc<Repo>,
}

impl StoryKeeperPostgres {
    /// Create a new postgres story keeper
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl StoryKeeper for StoryKeeperPostgres {
    /// Get a story
    async fn fetch(&self, id: Uuid) -> Result<Story> {
        self.repo.fetch_story(id).await
    }

    /// Get a page of stories
    async fn list(&self, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)> {
        self.repo.list_stories(cursor, limit).await
    }

    /// Create a story
    async fn create(&self, name: String) -> Result<Story> {
        self.repo.create_story(name).await
    }

    /// Update a story
    async fn update(&self, id: Uuid, name: String) -> Result<Story> {
        self.repo
            .fetch_story(id)
            .and_then(|_| self.repo.update_story(id, name))
            .await
    }

    /// Delete a story
    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo
            .fetch_story(id)
            .and_then(|_| self.repo.delete_story(id))
            .await
            .map(|_| ())
    }
}
