use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Create new stories.
pub struct CreateStory {
    pub repo: Arc<StoryRepo>,
}

#[async_trait]
impl UseCase for CreateStory {
    /// Input is a story name
    type Req = String;

    /// Output is a story
    type Rep = Result<Story>;

    /// Create a story
    async fn execute(&self, name: String) -> Result<Story> {
        self.repo.create(name).await
    }
}
