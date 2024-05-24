use crate::{
    domain::Task,
    repo::{StoryRepo, TaskRepo},
    service::UseCase,
    Result,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Get story tasks.
pub struct GetTasks {
    pub story_repo: Arc<StoryRepo>,
    pub task_repo: Arc<TaskRepo>,
}

#[async_trait]
impl UseCase for GetTasks {
    /// Input is a story id
    type Req = i32;

    /// Output is a vector of tasks
    type Rep = Result<Vec<Task>>;

    /// Get all tasks for a story if it exists. If the story does't exist,
    /// return a `NotFound` error.
    async fn execute(&self, story_id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: story_id={}", story_id);
        // Try and query for tasks first.
        let tasks = self.task_repo.list(story_id).await?;
        // When no tasks were returned, check whether the story exists.
        // This is an optimization; if tasks were returned, the story DOES exist
        // and no further querying is required.
        if tasks.is_empty() {
            let _ = self.story_repo.fetch(story_id).await?;
        }
        Ok(tasks)
    }
}
