use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Updates stories
pub struct UpdateStory {
    story_repo: Arc<StoryRepo>,
}

impl UpdateStory {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self { story_repo }
    }
}

#[async_trait]
impl UseCase for UpdateStory {
    /// Input is a story id and updated name
    type Req = (i32, String);

    /// Output is the updated story
    type Rep = Result<Story>;

    /// Update a story if it exists.
    async fn execute(&self, (id, name): Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}, name={}", id, name);

        self.story_repo
            .fetch(id)
            .and_then(|_| self.story_repo.update(id, name))
            .await
    }
}
