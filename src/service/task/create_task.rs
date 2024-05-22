use crate::{domain::Task, repo::TaskRepo, service::Service, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Create a new task.
pub struct CreateTask {
    repo: Arc<TaskRepo>,
}

impl CreateTask {
    /// Create a new service for creating stories.
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for CreateTask {
    /// Input is the story_id and task name
    type Req = (i32, String);

    /// Output is the new task
    type Rep = Result<Task>;

    /// Create a task
    async fn call(&self, (story_id, name): Self::Req) -> Self::Rep {
        tracing::debug!("execute: {}, {}", story_id, name);
        self.repo
            .fetch(story_id)
            .and_then(|_| self.repo.create(story_id, name))
            .await
    }
}
