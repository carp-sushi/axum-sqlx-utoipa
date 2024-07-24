use crate::{domain::Story, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get pages of stories.
pub struct GetStories {
    repo: Arc<Repo>,
}

impl GetStories {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for GetStories {
    /// Input is unit.
    type Req = (i32, i32);

    /// Output is vector of stories.
    type Rep = Result<(i32, Vec<Story>)>;

    /// Get a page of stories.
    async fn execute(&self, (page_id, page_size): Self::Req) -> Self::Rep {
        tracing::debug!("execute: page_id={}, page_size={}", page_id, page_size);

        let stories = self.repo.list_stories(page_id, page_size).await?;
        let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();

        Ok((next_page, stories))
    }
}
