use crate::{domain::Task, repo::TaskRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Create a new task.
pub struct CreateTask {
    pub repo: Arc<TaskRepo>,
}

#[async_trait]
impl UseCase for CreateTask {
    /// Input is the story_id and task name
    type Req = (i32, String);

    /// Output is the new task
    type Rep = Result<Task>;

    /// Create a task
    async fn execute(&self, (story_id, name): Self::Req) -> Self::Rep {
        tracing::debug!("execute: {}, {}", story_id, name);
        self.repo
            .fetch(story_id)
            .and_then(|_| self.repo.create(story_id, name))
            .await
    }
}
