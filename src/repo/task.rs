use super::Repo;
use crate::{
    domain::{Status, Task},
    Error, Result,
};
use uuid::Uuid;

// Put some reasonable upper limit when querying tasks for a story.
const MAX_TASKS: i64 = 100;

// Extend repo with queries related to tasks.
impl Repo {
    /// Get a task by id
    pub async fn fetch_task(&self, task_id: Uuid) -> Result<Task> {
        let query = sqlx::query_as!(
            Task,
            "SELECT id, story_id, name, status, created_at, updated_at FROM tasks WHERE id = $1",
            task_id,
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(task) => Ok(task),
            None => Err(Error::not_found(format!("task not found: {task_id}"))),
        }
    }

    /// Select tasks for a story
    pub async fn list_tasks(&self, story_id: Uuid) -> Result<Vec<Task>> {
        let query = sqlx::query_as!(
            Task,
            r#"SELECT id, story_id, name, status, created_at, updated_at FROM tasks
            WHERE story_id = $1 ORDER BY created_at LIMIT $2"#,
            story_id,
            MAX_TASKS,
        );
        let tasks = query.fetch_all(self.db_ref()).await?;
        Ok(tasks)
    }

    /// Insert a new task
    pub async fn create_task(&self, story_id: Uuid, name: String, status: Status) -> Result<Task> {
        let query = sqlx::query_as!(
            Task,
            r#"INSERT INTO tasks (story_id, name, status) VALUES ($1, $2, $3)
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            story_id,
            name,
            status.to_string(),
        );
        let task = query.fetch_one(self.db_ref()).await?;
        Ok(task)
    }

    /// Update task name and status.
    pub async fn update_task(&self, task_id: Uuid, name: String, status: Status) -> Result<Task> {
        let query = sqlx::query_as!(
            Task,
            r#"UPDATE tasks SET name = $1, status = $2, updated_at = now() WHERE id = $3
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            name,
            status.to_string(),
            task_id,
        );
        let task = query.fetch_one(self.db_ref()).await?;
        Ok(task)
    }

    /// Delete a task.
    pub async fn delete_task(&self, task_id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
            .execute(self.db_ref())
            .await?;
        Ok(())
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
        let image = Postgres::default().with_tag("17-alpine");
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(Arc::clone(&pool));

        // Set up a story to put tasks under
        let name = "Books To Read".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        let story_id = story.id;

        // Create task, ensuring status is incomplete
        let status = Status::Incomplete;
        let task = repo
            .create_task(story_id.clone(), "Suttree".to_string(), status)
            .await
            .unwrap();
        assert_eq!(task.status, Status::Incomplete.to_string());

        // Assert task exists
        assert!(repo.fetch_task(task.id).await.is_ok());

        // Set task status to complete
        repo.update_task(task.id, task.name, Status::Complete)
            .await
            .unwrap();

        // Fetch task and assert status was updated
        let task = repo.fetch_task(task.id).await.unwrap();
        assert_eq!(task.status, Status::Complete.to_string());

        // Query tasks for story.
        let tasks = repo.list_tasks(story_id).await.unwrap();
        assert_eq!(tasks.len(), 1);

        // Delete the task
        repo.delete_task(task.id).await.unwrap();

        // Assert task was deleted
        assert!(repo.fetch_task(task.id).await.is_err());

        // Cleanup
        repo.delete_story(story_id).await.unwrap();
    }
}
