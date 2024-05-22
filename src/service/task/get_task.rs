use crate::{domain::Task, repo::TaskRepo, service::Service, Result};
use async_trait::async_trait;
use std::sync::Arc;

/// Get tasks by id.
pub struct GetTask {
    repo: Arc<TaskRepo>,
}

impl GetTask {
    /// Create a new service for getting tasks by id.
    pub fn new(repo: Arc<TaskRepo>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Service for GetTask {
    /// Input is a task id
    type Req = i32;

    /// Output is a task
    type Rep = Result<Task>;

    /// Get a task
    async fn call(&self, id: Self::Req) -> Self::Rep {
        self.repo.fetch(id).await
    }
}
