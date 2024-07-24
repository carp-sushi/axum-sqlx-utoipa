use crate::{domain::Story, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get stories by id.
pub struct GetStory {
    repo: Arc<Repo>,
}

impl GetStory {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
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
        self.repo.fetch_story(id).await
    }
}
