use crate::{domain::Story, repo::Repo, Result};
use futures_util::TryFutureExt;
use std::ops::Deref;
use std::sync::Arc;

/// A high-level API for managaing stories.
pub struct StoryService {
    repo: Arc<Repo>,
}

impl StoryService {
    /// Create a new story service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    /// Delete a story
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.fetch_story(id)
            .and_then(|_| self.delete_story(id))
            .await
            .map(|_| ())
    }

    /// Get a page of stories
    pub async fn list(&self, page_id: i32, page_size: i32) -> Result<(i32, Vec<Story>)> {
        let stories = self.list_stories(page_id, page_size).await?;
        let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();
        Ok((next_page, stories))
    }

    /// Update a story
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        self.fetch_story(id)
            .and_then(|_| self.update_story(id, name))
            .await
    }
}

// Allow access to calls on the inner repo.
impl Deref for StoryService {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
