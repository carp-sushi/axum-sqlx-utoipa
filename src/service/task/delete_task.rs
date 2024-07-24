use crate::{repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Delete a task by id.
pub struct DeleteTask {
    repo: Arc<Repo>,
}

impl DeleteTask {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
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

        self.repo
            .fetch_task(id)
            .and_then(|_| self.repo.delete_task(id))
            .await
            .map(|_| ())
    }
}
