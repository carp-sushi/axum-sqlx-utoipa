use super::{sql, Repo};
use crate::{
    domain::{Status, Task},
    Error, Result,
};
use futures_util::TryStreamExt;
use sqlx::{postgres::PgRow, FromRow, Row};
use std::str::FromStr;
use stripmargin::StripMargin;
use uuid::Uuid;

// Put some reasonable upper limit when querying tasks for a story.
const MAX_TASKS: i16 = 500;

/// Map sqlx rows to task domain objects.
impl FromRow<'_, PgRow> for Task {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let story_id = row.try_get("story_id")?;
        let name = row.try_get("name")?;
        let status: String = row.try_get("status")?;
        // Manually convert to enum type
        let status = Status::from_str(&status).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;
        let created_at = row.try_get("created_at")?;
        let updated_at = row.try_get("updated_at")?;

        Ok(Self {
            id,
            story_id,
            name,
            status,
            created_at,
            updated_at,
        })
    }
}

// Extend repo with queries related to tasks.
impl Repo {
    /// Get a task by id
    pub async fn fetch_task(&self, id: Uuid) -> Result<Task> {
        let query = sql::task::FETCH.strip_margin();

        let task_opt = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(self.db_ref())
            .await?;

        match task_opt {
            Some(task) => Ok(task),
            None => Err(Error::not_found(format!("task not found: {id}"))),
        }
    }

    /// Select tasks for a story
    pub async fn list_tasks(&self, story_id: Uuid) -> Result<Vec<Task>> {
        let query = sql::task::LIST.strip_margin();

        let mut result_set = sqlx::query(&query)
            .bind(story_id)
            .bind(MAX_TASKS)
            .fetch(self.db_ref());

        let mut tasks = Vec::with_capacity(MAX_TASKS as usize);

        while let Some(row) = result_set.try_next().await? {
            let task = Task::from_row(&row)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Insert a new task
    pub async fn create_task(&self, story_id: Uuid, name: String, status: Status) -> Result<Task> {
        let query = sql::task::CREATE.strip_margin();

        let task = sqlx::query_as(&query)
            .bind(story_id)
            .bind(name)
            .bind(status.to_string())
            .fetch_one(self.db_ref())
            .await?;

        Ok(task)
    }

    /// Update task name and status.
    pub async fn update_task(&self, id: Uuid, name: String, status: Status) -> Result<Task> {
        let query = sql::task::UPDATE.strip_margin();

        let task = sqlx::query_as(&query)
            .bind(name)
            .bind(status.to_string())
            .bind(id)
            .fetch_one(self.db_ref())
            .await?;

        Ok(task)
    }

    /// Delete a task.
    pub async fn delete_task(&self, id: Uuid) -> Result<u64> {
        let query = sql::task::DELETE.strip_margin();
        let result = sqlx::query(&query).bind(id).execute(self.db_ref()).await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::Status,
        repo::{tests, Repo},
    };
    use std::sync::Arc;

    use testcontainers::{runners::AsyncRunner, ImageExt};
    use testcontainers_modules::postgres::Postgres;

    #[ignore]
    #[tokio::test]
    async fn integration_test() {
        // Set up postgres test container backed repo
        let image = Postgres::default().with_tag("16-alpine");
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(Arc::clone(&pool));

        // Set up a story to put tasks under
        let name = "Books To Read".to_string();
        let story_id = repo.create_story(name.clone()).await.unwrap().id;

        // Create task, ensuring status is incomplete
        let status = Status::Incomplete;
        let task = repo
            .create_task(story_id.clone(), "Suttree".to_string(), status)
            .await
            .unwrap();
        assert_eq!(task.status, Status::Incomplete);

        // Assert task exists
        assert!(repo.fetch_task(task.id).await.is_ok());

        // Set task status to complete
        repo.update_task(task.id, task.name, Status::Complete)
            .await
            .unwrap();

        // Fetch task and assert status was updated
        let task = repo.fetch_task(task.id).await.unwrap();
        assert_eq!(task.status, Status::Complete);

        // Query tasks for story.
        let tasks = repo.list_tasks(story_id.clone()).await.unwrap();
        assert_eq!(tasks.len(), 1);

        // Delete the task
        let rows = repo.delete_task(task.id).await.unwrap();
        assert_eq!(rows, 1);

        // Assert task was deleted
        assert!(repo.fetch_task(task.id).await.is_err());

        // Cleanup
        assert!(repo.delete_story(story_id).await.unwrap() > 0);
    }
}
