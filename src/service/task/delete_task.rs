use crate::{repo::TaskRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete a task by id.
pub struct DeleteTask {
    task_repo: Arc<TaskRepo>,
}

impl DeleteTask {
    /// Constructor
    pub fn new(task_repo: Arc<TaskRepo>) -> Self {
        Self { task_repo }
    }
}

#[async_trait]
impl UseCase for DeleteTask {
    /// Input is a task id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a task.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);

        self.task_repo
            .fetch(id)
            .and_then(|_| self.task_repo.delete(id))
            .await
            .map(|_| ())
    }
}
