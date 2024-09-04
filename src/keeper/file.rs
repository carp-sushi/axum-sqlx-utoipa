use crate::{domain::StoryFile, repo::Repo, Result};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// File metadata persistence API
#[async_trait::async_trait]
pub trait FileKeeper: Send + Sync {
    /// Fetch a file
    async fn fetch(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile>;

    /// List all files for a story
    async fn list(&self, story_id: Uuid) -> Result<Vec<StoryFile>>;

    /// Create a file
    async fn create(
        &self,
        story_id: Uuid,
        storage_id: Uuid,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile>;

    /// Delete a file
    async fn delete(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile>;
}

/// Concrete persistence API for files using postgres
pub struct FileKeeperPostgres {
    repo: Arc<Repo>,
}

impl FileKeeperPostgres {
    /// Create a new postgres file keeper
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl FileKeeper for FileKeeperPostgres {
    /// Get a file
    async fn fetch(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile> {
        self.repo.fetch_file(story_id, file_id).await
    }

    /// Get all files for a story
    async fn list(&self, story_id: Uuid) -> Result<Vec<StoryFile>> {
        let files = self.repo.list_files(story_id).await?;
        if files.is_empty() {
            let _ = self.repo.fetch_story(story_id).await?;
        }
        Ok(files)
    }

    /// Create a file
    async fn create(
        &self,
        story_id: Uuid,
        storage_id: Uuid,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile> {
        self.repo
            .fetch_story(story_id)
            .and_then(|s| {
                self.repo
                    .create_file(s.id, storage_id, name, size, content_type)
            })
            .await
    }

    /// Delete a file
    async fn delete(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile> {
        let file = self.repo.fetch_file(story_id, file_id).await?;
        self.repo.delete_file(file_id).await?;
        Ok(file)
    }
}
