use crate::{repo::StoryRepo, service::Service, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete stories by id.
pub struct DeleteStory {
    repo: Arc<StoryRepo>,
}

impl DeleteStory {
    /// Create a new service for deleting stories by id.
    pub fn new(repo: Arc<StoryRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for DeleteStory {
    /// Input is a story id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a story.
    async fn call(&self, id: Self::Req) -> Self::Rep {
        let rows = self
            .repo
            .fetch(id)
            .and_then(|_| self.repo.delete(id))
            .await?;
        tracing::debug!("deleted {} stories", rows);
        Ok(())
    }
}
