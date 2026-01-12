use super::Repo;
use crate::{
    domain::{Status, StoryId, Task, TaskId},
    Error, Result,
};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

// Put some reasonable upper limit when querying tasks for a story.
const MAX_TASKS: i64 = 100;

/// The task entity object - used for query validation against the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TaskEntity {
    id: Uuid,
    story_id: Uuid,
    name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// The repo should map the entity to the domain object in public functions.
impl From<TaskEntity> for Task {
    fn from(entity: TaskEntity) -> Self {
        Self {
            id: TaskId(entity.id),
            story_id: StoryId(entity.story_id),
            name: entity.name,
            status: Status::from_str(&entity.status).unwrap_or_default(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

// Extend repo with queries related to tasks.
impl Repo {
    /// Get a task by id
    pub async fn fetch_task(&self, &TaskId(task_id): &TaskId) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            "SELECT id, story_id, name, status, created_at, updated_at FROM tasks WHERE id = $1",
            task_id,
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(entity) => Ok(Task::from(entity)),
            None => Err(Error::not_found(format!("task not found: {task_id}"))),
        }
    }

    /// Select tasks for a story
    pub async fn list_tasks(&self, &StoryId(story_id): &StoryId) -> Result<Vec<Task>> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"SELECT id, story_id, name, status, created_at, updated_at FROM tasks
            WHERE story_id = $1 ORDER BY created_at LIMIT $2"#,
            story_id,
            MAX_TASKS,
        );
        let entities = query.fetch_all(self.db_ref()).await?;
        Ok(entities.into_iter().map(Task::from).collect())
    }

    /// Insert a new task
    pub async fn create_task(
        &self,
        &StoryId(story_id): &StoryId,
        name: impl Into<String>,
        status: Status,
    ) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"INSERT INTO tasks (story_id, name, status) VALUES ($1, $2, $3)
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            story_id,
            name.into(),
            status.to_string(),
        );
        let entity = query.fetch_one(self.db_ref()).await?;
        Ok(Task::from(entity))
    }

    /// Update task name and status.
    pub async fn update_task(
        &self,
        &TaskId(task_id): &TaskId,
        name: impl Into<String>,
        status: Status,
    ) -> Result<Task> {
        let query = sqlx::query_as!(
            TaskEntity,
            r#"UPDATE tasks SET name = $1, status = $2 WHERE id = $3
            RETURNING id, story_id, name, status, created_at, updated_at"#,
            name.into(),
            status.to_string(),
            task_id,
        );
        let entity = query.fetch_one(self.db_ref()).await?;
        Ok(Task::from(entity))
    }

    /// Delete a task.
    pub async fn delete_task(&self, &TaskId(task_id): &TaskId) -> Result<()> {
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
        let image = Postgres::default().with_tag(tests::PG_VERSION_TAG);
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(Arc::clone(&pool));

        // Set up a story to put tasks under
        let story = repo.create_story("Books To Read").await.unwrap();
        let story_id = story.id;

        // Create a task
        let task = repo
            .create_task(&story_id, "Suttree", Status::Incomplete)
            .await
            .unwrap();
        assert_eq!(task.name, "Suttree");

        // Query tasks for story.
        let tasks = repo.list_tasks(&story_id).await.unwrap();
        assert_eq!(tasks.len(), 1);

        // Set task status to complete
        repo.update_task(&task.id, task.name, Status::Complete)
            .await
            .unwrap();
        assert_eq!(
            repo.fetch_task(&task.id).await.unwrap().status,
            Status::Complete
        );

        // Delete the task
        repo.delete_task(&task.id).await.unwrap();
        assert!(repo.fetch_task(&task.id).await.is_err());
    }
}
