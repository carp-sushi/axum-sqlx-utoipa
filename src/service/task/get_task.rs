use crate::{domain::Task, repo::Repo, service::UseCase, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get tasks by id.
pub struct GetTask {
    repo: Arc<Repo>,
}

impl GetTask {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UseCase for GetTask {
    /// Input is a task id
    type Req = i32;

    /// Output is a task
    type Rep = Result<Task>;

    /// Get a task
    async fn execute(&self, id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);
        self.repo.fetch_task(id).await
    }
}
