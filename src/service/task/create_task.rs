use crate::{
    domain::Task,
    repo::{StoryRepo, TaskRepo},
    service::UseCase,
    Result,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Create a new task.
pub struct CreateTask {
    story_repo: Arc<StoryRepo>,
    task_repo: Arc<TaskRepo>,
}

impl CreateTask {
    /// Constructor
    pub fn new(story_repo: Arc<StoryRepo>, task_repo: Arc<TaskRepo>) -> Self {
        Self {
            story_repo,
            task_repo,
        }
    }
}

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
        self.story_repo
            .fetch(story_id)
            .and_then(|_| self.task_repo.create(story_id, name))
            .await
    }
}
