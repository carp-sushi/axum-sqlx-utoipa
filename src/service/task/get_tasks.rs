use crate::{domain::Task, repo::TaskRepo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get story tasks.
pub struct GetTasks {
    pub repo: Arc<TaskRepo>,
}

#[async_trait]
impl UseCase for GetTasks {
    /// Input is a story id
    type Req = i32;

    /// Output is a vector of tasks
    type Rep = Result<Vec<Task>>;

    /// Get all tasks for a story.
    async fn execute(&self, story_id: Self::Req) -> Self::Rep {
        self.repo.fetch_all(story_id).await
    }
}
