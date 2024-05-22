use crate::{domain::Story, repo::StoryRepo, service::Service, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get a list of recent stories.
pub struct GetStories {
    repo: Arc<StoryRepo>,
}

impl GetStories {
    /// Create a new service for getting recent stories.
    pub fn new(repo: Arc<StoryRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for GetStories {
    /// Input is unit.
    type Req = ();

    /// Output is vector of stories.
    type Rep = Result<Vec<Story>>;

    /// Get a page of stories.
    async fn call(&self, _: Self::Req) -> Self::Rep {
        self.repo.list_recent().await
    }
}
