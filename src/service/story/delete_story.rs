use crate::{repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete stories by id.
pub struct DeleteStory {
    pub repo: Arc<StoryRepo>,
}

#[async_trait]
impl UseCase for DeleteStory {
    /// Input is a story id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a story.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        let rows = self
            .repo
            .fetch(id)
            .and_then(|_| self.repo.delete(id))
            .await?;
        tracing::debug!("deleted {} rows", rows);
        Ok(())
    }
}
