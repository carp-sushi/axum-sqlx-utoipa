use crate::{repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete stories by id.
pub struct DeleteStory {
    repo: Arc<Repo>,
}

impl DeleteStory {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for DeleteStory {
    /// Input is a story id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a story.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);

        self.repo
            .fetch_story(id)
            .and_then(|_| self.repo.delete_story(id))
            .await
            .map(|_| ())
    }
}
