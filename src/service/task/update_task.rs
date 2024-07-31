use crate::{
    domain::{Status, Task},
    repo::Repo,
    service::UseCase,
    Error, Result,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::{ops::Deref, sync::Arc};

/// Update tasks.
pub struct UpdateTask(pub Arc<Repo>);

#[async_trait]
impl UseCase for UpdateTask {
    /// Input is a task id, updated name and/or status
    type Req = (i32, Option<String>, Option<Status>);

    /// Output is the updated task
    type Rep = Result<Task>;

    /// Update a task if it exists.
    async fn execute(&self, (id, name_opt, status_opt): Self::Req) -> Self::Rep {
        tracing::debug!("execute: id={}", id);

        // Make sure an update was provided.
        if name_opt.is_none() && status_opt.is_none() {
            let error = Error::invalid_args("no task updates provided");
            return Err(error);
        }

        // Fetch the task and update it.
        self.fetch_task(id)
            .and_then(|task| {
                let name = name_opt.unwrap_or(task.name);
                let status = status_opt.unwrap_or(task.status);
                self.update_task(id, name, status)
            })
            .await
    }
}

// Allows calls to wrapped repo at use case level.
impl Deref for UpdateTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
