use crate::{repo::TaskRepo, service::Service, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete a task by id.
pub struct DeleteTask {
    repo: Arc<TaskRepo>,
}

impl DeleteTask {
    /// Create a new service for deleting a task by id.
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for DeleteTask {
    /// Input is a task id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a task.
    async fn call(&self, id: Self::Req) -> Self::Rep {
        let rows = self
            .repo
            .fetch(id)
            .and_then(|_| self.repo.delete(id))
            .await?;
        tracing::debug!("deleted {} tasks", rows);
        Ok(())
    }
}
