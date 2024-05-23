use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get a list of recent stories.
pub struct GetStories {
    pub repo: Arc<StoryRepo>,
}

#[async_trait]
impl UseCase for GetStories {
    /// Input is unit.
    type Req = ();

    /// Output is vector of stories.
    type Rep = Result<Vec<Story>>;

    /// Get a page of stories.
    async fn execute(&self, _: Self::Req) -> Self::Rep {
        self.repo.list_recent().await
    }
}
