use crate::{domain::Task, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::{ops::Deref, sync::Arc};

/// Get a task by id.
pub struct GetTask(pub Arc<Repo>);

#[async_trait]
impl UseCase for GetTask {
    /// Input is a task id
    type Req = i32;

    /// Output is a task or error.
    type Rep = Result<Task>;

    /// Get a task.
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);
        self.fetch_task(id).await
    }
}

// Allows calls to wrapped repo at use case level.
impl Deref for GetTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
