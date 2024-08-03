use crate::{
    domain::{Status, Task},
    repo::Repo,
    Result,
};
use futures_util::TryFutureExt;
use std::ops::Deref;
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
        self.fetch_story(story_id)
            .and_then(|_| self.create_task(story_id, name))
            .await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.fetch_task(id)
            .and_then(|_| self.delete_task(id))
            .await
            .map(|_| ())
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
        // Try and query for tasks first.
        let tasks = self.list_tasks(story_id).await?;
        // When zero tasks were returned, check whether the story exists.
        // This is an optimization; if tasks were returned, the story DOES exist
        // and no further querying is required.
        if tasks.is_empty() {
            // Only care about errors here
            let _ = self.fetch_story(story_id).await?;
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
        self.fetch_task(id)
            .and_then(|task| {
                let name = name_opt.unwrap_or(task.name);
                let status = status_opt.unwrap_or(task.status);
                self.update_task(id, name, status)
            })
            .await
    }
}

// Allow access to calls on the inner repo.
impl Deref for TaskService {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
