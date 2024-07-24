use crate::{domain::Task, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get story tasks.
pub struct GetTasks {
    repo: Arc<Repo>,
}

impl GetTasks {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
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
        let tasks = self.repo.list_tasks(story_id).await?;

        // When zero tasks were returned, check whether the story exists.
        // This is an optimization; if tasks were returned, the story DOES exist
        // and no further querying is required.
        if tasks.is_empty() {
            // Only care about errors here
            let _ = self.repo.fetch_story(story_id).await?;
        }

        Ok(tasks)
    }
}
