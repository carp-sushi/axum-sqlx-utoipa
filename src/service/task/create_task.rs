use crate::{domain::Task, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::{ops::Deref, sync::Arc};

/// Create a new task.
pub struct CreateTask(pub Arc<Repo>);

#[async_trait]
impl UseCase for CreateTask {
    /// Input is the story_id and task name
    type Req = (i32, String);

    /// Output is the new task
    type Rep = Result<Task>;

    /// Create a task
    async fn execute(&self, (story_id, name): Self::Req) -> Self::Rep {
        tracing::debug!("execute: story_id={}, name={}", story_id, name);

        // Verify the parent story exists, then create the task
        self.fetch_story(story_id)
            .and_then(|_| self.create_task(story_id, name))
            .await
    }
}

// Allows calls to wrapped repo at use case level.
impl Deref for CreateTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
