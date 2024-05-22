use crate::{
    domain::{Status, Task},
    repo::TaskRepo,
    service::Service,
    Result,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Update tasks.
pub struct UpdateTask {
    repo: Arc<TaskRepo>,
}

impl UpdateTask {
    /// Create a new service for updating tasks.
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for UpdateTask {
    /// Input is a task id, updated name and/or status
    type Req = (i32, Option<String>, Option<Status>);

    /// Output is the updated task
    type Rep = Result<Task>;

    /// Update a task if it exists.
    async fn call(&self, (id, name_opt, status_opt): Self::Req) -> Self::Rep {
        let existing = self.repo.fetch(id).await?;
        self.repo
            .update(
                id,
                name_opt.unwrap_or(existing.name),
                status_opt.unwrap_or(existing.status),
            )
            .await
    }
}
