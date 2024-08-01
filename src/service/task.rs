use crate::{
    domain::{Status, Task},
    repo::Repo,
    Error, Result,
};
use futures_util::TryFutureExt;
use std::sync::Arc;

/// A high-level API for managaing tasks.
pub struct TaskService {
    repo: Arc<Repo>,
}

impl TaskService {
    /// Create a new task service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }

    /// Create a task
    pub async fn create(&self, story_id: i32, name: String) -> Result<Task> {
        self.repo
            .fetch_story(story_id)
            .and_then(|_| self.repo.create_task(story_id, name))
            .await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.repo
            .fetch_task(id)
            .and_then(|_| self.repo.delete_task(id))
            .await
            .map(|_| ())
    }

    /// Get a task
    pub async fn get(&self, id: i32) -> Result<Task> {
        self.repo.fetch_task(id).await
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
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

    /// Update a task
    pub async fn update(
        &self,
        id: i32,
        name_opt: Option<String>,
        status_opt: Option<Status>,
    ) -> Result<Task> {
        // Make sure an update was provided.
        if name_opt.is_none() && status_opt.is_none() {
            let error = Error::invalid_args("no task updates provided");
            return Err(error);
        }
        // Fetch the task and update it.
        self.repo
            .fetch_task(id)
            .and_then(|task| {
                let name = name_opt.unwrap_or(task.name);
                let status = status_opt.unwrap_or(task.status);
                self.repo.update_task(id, name, status)
            })
            .await
    }
}
