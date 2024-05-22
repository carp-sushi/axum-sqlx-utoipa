use crate::{domain::Task, repo::TaskRepo, service::Service, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get story tasks.
pub struct GetTasks {
    repo: Arc<TaskRepo>,
}

impl GetTasks {
    /// Create a new service for getting story tasks.
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for GetTasks {
    /// Input is a story id
    type Req = i32;

    /// Output is a vector of tasks
    type Rep = Result<Vec<Task>>;

    /// Get all tasks for a story.
    async fn call(&self, story_id: Self::Req) -> Self::Rep {
        self.repo.fetch_all(story_id).await
    }
}
