use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Create new stories.
pub struct CreateStory {
    story_repo: Arc<StoryRepo>,
}

impl CreateStory {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self { story_repo }
    }
}

#[async_trait]
impl UseCase for CreateStory {
    /// Input is a story name
    type Req = String;

    /// Output is a story
    type Rep = Result<Story>;

    /// Create a story
    async fn execute(&self, name: String) -> Result<Story> {
        tracing::debug!("execute: name={}", name);
        self.story_repo.create(name).await
    }
}
