use crate::{domain::Story, repo::StoryRepo, service::Service, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Create new stories.
pub struct CreateStory {
    repo: Arc<StoryRepo>,
}

impl CreateStory {
    /// Create a new service for creating stories.
    pub fn new(repo: Arc<StoryRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for CreateStory {
    /// Input is a story name
    type Req = String;

    /// Output is a story
    type Rep = Result<Story>;

    /// Create a story
    async fn call(&self, name: String) -> Result<Story> {
        self.repo.create(name).await
    }
}
