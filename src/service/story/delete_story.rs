use crate::{repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete stories by id.
pub struct DeleteStory {
    story_repo: Arc<StoryRepo>,
}

impl DeleteStory {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self { story_repo }
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

        self.story_repo
            .fetch(id)
            .and_then(|_| self.story_repo.delete(id))
            .await
            .map(|_| ())
    }
}
