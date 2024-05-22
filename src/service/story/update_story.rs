use crate::{domain::Story, repo::StoryRepo, service::Service, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Updates stories
pub struct UpdateStory {
    repo: Arc<StoryRepo>,
}

impl UpdateStory {
    /// Create a new service for updating stories.
    pub fn new(repo: Arc<StoryRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for UpdateStory {
    /// Input is a story id and updated name
    type Req = (i32, String);

    /// Output is the updated story
    type Rep = Result<Story>;

    /// Update a story if it exists.
    async fn call(&self, (id, name): Self::Req) -> Self::Rep {
        self.repo
            .fetch(id)
            .and_then(|_| self.repo.update(id, name))
            .await
    }
}
