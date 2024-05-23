use super::UseCase;
use crate::{
    domain::{Status, Task},
    repo::TaskRepo,
    Result,
};
use std::sync::Arc;

mod create_task;
mod delete_task;
mod get_task;
mod get_tasks;
mod update_task;

use create_task::CreateTask;
use delete_task::DeleteTask;
use get_task::GetTask;
use get_tasks::GetTasks;
use update_task::UpdateTask;

/// Aggregates task use cases into a unified api.
pub struct TaskService {
    create_task: CreateTask,
    delete_task: DeleteTask,
    get_task: GetTask,
    get_tasks: GetTasks,
    update_task: UpdateTask,
}

impl TaskService {
    /// Create a new task service
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self {
            create_task: CreateTask { repo: repo.clone() },
            delete_task: DeleteTask { repo: repo.clone() },
            get_task: GetTask { repo: repo.clone() },
            get_tasks: GetTasks { repo: repo.clone() },
            update_task: UpdateTask { repo },
        }
    }

    /// Create a task
    pub async fn create(&self, story_id: i32, name: String) -> Result<Task> {
        self.create_task.execute((story_id, name)).await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.delete_task.execute(id).await
    }

    /// Get a task
    pub async fn get(&self, id: i32) -> Result<Task> {
        self.get_task.execute(id).await
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
        self.get_tasks.execute(story_id).await
    }

    /// Update a task
    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        status: Option<Status>,
    ) -> Result<Task> {
        self.update_task.execute((id, name, status)).await
    }
}
