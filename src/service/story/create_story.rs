use crate::{domain::Story, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Create new stories.
pub struct CreateStory {
    repo: Arc<Repo>,
}

impl CreateStory {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
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
        self.repo.create_story(name).await
    }
}
