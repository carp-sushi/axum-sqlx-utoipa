use crate::{domain::Story, repo::StoryRepo, service::Service, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get stories by id.
pub struct GetStory {
    repo: Arc<StoryRepo>,
}

impl GetStory {
    /// Create a new service for getting stories;
    pub fn new(repo: Arc<StoryRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for GetStory {
    /// Input is the story id
    type Req = i32;

    /// Output is the story
    type Rep = Result<Story>;

    /// Get a story if it exists
    async fn call(&self, id: i32) -> Result<Story> {
        self.repo.fetch(id).await
    }
}
