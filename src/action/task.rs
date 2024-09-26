use crate::{
    api::Ctx,
    domain::{Status, Task},
    Result,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// Get a task
pub struct GetTask;
impl GetTask {
    pub async fn execute(ctx: Arc<Ctx>, task_id: Uuid) -> Result<Task> {
        ctx.repo.fetch_task(task_id).await
    }
}

/// Get all tasks for a story.
pub struct GetTasks;
impl GetTasks {
    pub async fn execute(
        ctx: Arc<Ctx>,
        story_id: Uuid,
        status: Option<Status>,
    ) -> Result<Vec<Task>> {
        let mut tasks = ctx
            .repo
            .fetch_story(story_id)
            .and_then(|s| ctx.repo.list_tasks(s.id))
            .await?;
        if let Some(status) = status {
            let status = status.to_string();
            tasks.retain(|t| t.status == status);
        }
        Ok(tasks)
    }
}

/// Create a task
pub struct CreateTask;
impl CreateTask {
    pub async fn execute(
        ctx: Arc<Ctx>,
        story_id: Uuid,
        name: String,
        status: Status,
    ) -> Result<Task> {
        ctx.repo
            .fetch_story(story_id)
            .and_then(|s| ctx.repo.create_task(s.id, name, status))
            .await
    }
}

/// Update name and/or status of a task.
pub struct UpdateTask;
impl UpdateTask {
    pub async fn execute(
        ctx: Arc<Ctx>,
        task_id: Uuid,
        name: Option<String>,
        status: Option<Status>,
    ) -> Result<Task> {
        let task = ctx
            .repo
            .fetch_task(task_id)
            .and_then(|t| {
                let status = status.unwrap_or(t.status());
                let name = name.unwrap_or(t.name);
                ctx.repo.update_task(task_id, name, status)
            })
            .await?;
        Ok(task)
    }
}

/// Delete a task
pub struct DeleteTask;
impl DeleteTask {
    pub async fn execute(ctx: Arc<Ctx>, task_id: Uuid) -> Result<()> {
        ctx.repo
            .fetch_task(task_id)
            .and_then(|t| ctx.repo.delete_task(t.id))
            .await
    }
}
