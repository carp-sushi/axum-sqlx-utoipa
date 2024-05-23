use crate::{repo::TaskRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete a task by id.
pub struct DeleteTask {
    pub repo: Arc<TaskRepo>,
}

#[async_trait]
impl UseCase for DeleteTask {
    /// Input is a task id
    type Req = i32;

    /// Output is unit
    type Rep = Result<()>;

    /// Delete a task.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        let rows = self
            .repo
            .fetch(id)
            .and_then(|_| self.repo.delete(id))
            .await?;
        tracing::debug!("deleted {} rows", rows);
        Ok(())
    }
}
