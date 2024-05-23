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
    type Req = i32;

    /// Output is vector of stories.
    type Rep = Result<(i32, Vec<Story>)>;

    /// Get a page of stories.
    async fn execute(&self, page_id: Self::Req) -> Self::Rep {
        let stories = self.repo.list(page_id).await?;
        let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();
        Ok((next_page, stories))
    }
}
