use crate::{
    domain::{Status, Task},
    repo::Repo,
    Result,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// Task persistence API
#[async_trait::async_trait]
pub trait TaskKeeper: Send + Sync {
    /// Fetch a task by ID
    async fn fetch(&self, id: Uuid) -> Result<Task>;

    /// Get tasks for a story
    async fn list(&self, story_id: Uuid, status: Option<Status>) -> Result<Vec<Task>>;

    /// Create a new task
    async fn create(&self, story_id: Uuid, name: String, status: Status) -> Result<Task>;

    /// Update an existing task
    async fn update(&self, id: Uuid, name: Option<String>, status: Option<Status>) -> Result<Task>;

    /// Delete a task
    async fn delete(&self, id: Uuid) -> Result<()>;
}

/// Concrete persistence API for tasks using postgres
pub struct TaskKeeperPostgres {
    repo: Arc<Repo>,
}

impl TaskKeeperPostgres {
    /// Create a new postgres task keeper
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl TaskKeeper for TaskKeeperPostgres {
    /// Get a task
    async fn fetch(&self, id: Uuid) -> Result<Task> {
        self.repo.fetch_task(id).await
    }

    /// Get tasks for  a story
    async fn list(&self, story_id: Uuid, status: Option<Status>) -> Result<Vec<Task>> {
        let mut tasks = self.repo.list_tasks(story_id).await?;
        if tasks.is_empty() {
            let _ = self.repo.fetch_story(story_id).await?;
        }
        if let Some(status) = status {
            tasks.retain(|t| t.status == status.to_string());
        }
        Ok(tasks)
    }

    /// Create a task
    async fn create(&self, story_id: Uuid, name: String, status: Status) -> Result<Task> {
        self.repo
            .fetch_story(story_id)
            .and_then(|_| self.repo.create_task(story_id, name, status))
            .await
    }

    /// Update a task
    async fn update(&self, id: Uuid, name: Option<String>, status: Option<Status>) -> Result<Task> {
        self.repo
            .fetch_task(id)
            .and_then(|t| {
                self.repo.update_task(
                    t.id,
                    name.unwrap_or(t.clone().name),
                    status.unwrap_or(t.status()),
                )
            })
            .await
    }

    /// Delete a task
    async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo
            .fetch_task(id)
            .and_then(|_| self.repo.delete_task(id))
            .await
            .map(|_| ())
    }
}
