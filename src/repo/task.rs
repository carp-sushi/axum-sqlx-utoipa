use super::Repo;
use crate::{
    domain::{Status, Task},
    Error, Result,
};
use futures_util::TryStreamExt;
use sqlx::{postgres::PgRow, FromRow, Row};
use std::str::FromStr;

/// Map sqlx rows to task domain objects.
impl FromRow<'_, PgRow> for Task {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        // Extract column values
        let id = row.try_get("id")?;
        let story_id = row.try_get("story_id")?;
        let name = row.try_get("name")?;
        let status: String = row.try_get("status")?;

        // Convert to enum type
        let status = Status::from_str(&status).map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        // Task
        Ok(Self {
            id,
            story_id,
            name,
            status,
        })
    }
}

// Extend repo with queries related to tasks.
impl Repo {
    /// Get a task by id
    pub async fn fetch_task(&self, id: i32) -> Result<Task> {
        tracing::debug!("fetch: id={}", id);

        let q = sqlx::query_as("SELECT id, story_id, name, status FROM tasks WHERE id = $1");
        let task_option = q.bind(id).fetch_optional(self.db_ref()).await?;

        match task_option {
            Some(task) => Ok(task),
            None => Err(Error::not_found(format!("task not found: {}", id))),
        }
    }

    /// Select tasks for a story
    pub async fn list_tasks(&self, story_id: i32) -> Result<Vec<Task>> {
        tracing::debug!("list: story_id={}", story_id);

        let q = sqlx::query(
            r#"SELECT id, story_id, name, status FROM tasks WHERE story_id = $1
            ORDER BY id LIMIT 1000"#,
        );
        let mut result_set = q.bind(story_id).fetch(self.db_ref());

        // Assume a hard limit on the number of tasks under a story.
        let mut tasks = Vec::with_capacity(1000);
        while let Some(row) = result_set.try_next().await? {
            let task = Task::from_row(&row)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Insert a new task
    pub async fn create_task(&self, story_id: i32, name: String) -> Result<Task> {
        tracing::debug!("create: story_id={}, name={}", story_id, name);

        let q = sqlx::query_as(
            r#"INSERT INTO tasks (story_id, name)
            VALUES ($1, $2) RETURNING id, story_id, name, status"#,
        );
        let task = q.bind(story_id).bind(name).fetch_one(self.db_ref()).await?;

        Ok(task)
    }

    /// Update task name and status.
    pub async fn update_task(&self, id: i32, name: String, status: Status) -> Result<Task> {
        tracing::debug!("update: id={}, name={}, status={}", id, name, status);

        let q = sqlx::query_as(
            r#"UPDATE tasks SET name = $1, status = $2 WHERE id = $3
            RETURNING id, story_id, name, status"#,
        );

        let task = q
            .bind(name)
            .bind(status.to_string())
            .bind(id)
            .fetch_one(self.db_ref())
            .await?;

        Ok(task)
    }

    /// Delete a task.
    pub async fn delete_task(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete: id={}", id);

        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(self.db_ref())
            .await?;

        Ok(result.rows_affected())
    }

    /// Check whether a task exists.
    pub async fn task_exists(&self, id: i32) -> bool {
        tracing::debug!("exists: id={}", id);

        let q = sqlx::query("SELECT EXISTS(SELECT 1 FROM tasks WHERE id = $1)");
        let result = q.bind(id).fetch_one(self.db_ref()).await;

        if let Ok(row) = result {
            row.try_get::<bool, _>("exists").unwrap_or_default()
        } else {
            false
        }
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
        let task = repo
            .create_task(story_id.clone(), "Suttree".to_string())
            .await
            .unwrap();
        assert_eq!(task.status, Status::Incomplete);

        // Assert task exists
        assert!(repo.task_exists(task.id).await);

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
        assert!(!repo.task_exists(task.id).await);

        // Cleanup
        assert!(repo.delete_story(story_id).await.unwrap() > 0);
    }
}
