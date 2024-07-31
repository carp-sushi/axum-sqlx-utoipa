use crate::{repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::{ops::Deref, sync::Arc};

/// Delete a task by id.
pub struct DeleteTask(pub Arc<Repo>);

#[async_trait]
impl UseCase for DeleteTask {
    /// Input is a task id
    type Req = i32;

    /// Output is unit or error
    type Rep = Result<()>;

    /// Delete a task.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);
        self.fetch_task(id)
            .and_then(|_| self.delete_task(id))
            .await
            .map(|_| ())
    }
}

// Allows calls to wrapped repo at use case level.
impl Deref for DeleteTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
