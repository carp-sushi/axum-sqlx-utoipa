use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get stories by id.
pub struct GetStory {
    story_repo: Arc<StoryRepo>,
}

impl GetStory {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self { story_repo }
    }
}

#[async_trait]
impl UseCase for GetStory {
    /// Input is the story id
    type Req = i32;

    /// Output is the story
    type Rep = Result<Story>;

    /// Get a story if it exists
    async fn execute(&self, id: i32) -> Result<Story> {
        tracing::debug!("execute: id={}", id);
        self.story_repo.fetch(id).await
    }
}
