use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get pages of stories.
pub struct GetStories {
    story_repo: Arc<StoryRepo>,
}

impl GetStories {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self { story_repo }
    }
}

#[async_trait]
impl UseCase for GetStories {
    /// Input is unit.
    type Req = i32;

    /// Output is vector of stories.
    type Rep = Result<(i32, Vec<Story>)>;

    /// Get a page of stories.
    async fn execute(&self, page_id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: page_id={}", page_id);

        let stories = self.story_repo.list(page_id).await?;
        let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();

        Ok((next_page, stories))
    }
}
