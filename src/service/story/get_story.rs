use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get stories by id.
pub struct GetStory {
    pub repo: Arc<StoryRepo>,
}

#[async_trait]
impl UseCase for GetStory {
    /// Input is the story id
    type Req = i32;

    /// Output is the story
    type Rep = Result<Story>;

    /// Get a story if it exists
    async fn execute(&self, id: i32) -> Result<Story> {
        self.repo.fetch(id).await
    }
}
