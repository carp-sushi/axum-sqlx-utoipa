use crate::{domain::Story, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Updates stories
pub struct UpdateStory {
    repo: Arc<Repo>,
}

impl UpdateStory {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
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

        self.repo
            .fetch_story(id)
            .and_then(|_| self.repo.update_story(id, name))
            .await
    }
}
