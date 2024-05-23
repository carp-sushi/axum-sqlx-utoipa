use crate::{domain::Story, Error, Result};
use futures_util::TryStreamExt;
use sqlx::{
    postgres::{PgPool, PgRow},
    FromRow, Row,
};
use std::sync::Arc;

/// Map sqlx rows to story domain objects.
impl FromRow<'_, PgRow> for Story {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

/// Concrete story related database logic
pub struct StoryRepo {
    db: Arc<PgPool>,
}

impl StoryRepo {
    /// Constructor
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    /// Get a ref to the connection pool.
    fn db_ref(&self) -> &PgPool {
        self.db.as_ref()
    }
}

impl StoryRepo {
    /// Select a story by id
    pub async fn fetch(&self, id: i32) -> Result<Story> {
        tracing::debug!("fetch: id={}", id);

        let q = sqlx::query_as("SELECT id, name FROM stories WHERE id = $1");
        let maybe_story = q.bind(id).fetch_optional(self.db_ref()).await?;

        match maybe_story {
            Some(story) => Ok(story),
            None => Err(Error::NotFound {
                message: format!("story not found: {}", id),
            }),
        }
    }

    /// Select a page of stories.
    pub async fn list(&self, page_id: i32) -> Result<Vec<Story>> {
        tracing::debug!("list: page_id={}", page_id);

        let q = sqlx::query(
            r#"SELECT id, name FROM stories WHERE id <= $1
            ORDER BY id desc LIMIT 100"#,
        );
        let mut result_set = q.bind(page_id).fetch(self.db_ref());

        let mut result = Vec::with_capacity(100);
        while let Some(row) = result_set.try_next().await? {
            let story = Story::from_row(&row)?;
            result.push(story);
        }

        Ok(result)
    }

    /// Insert a new story
    pub async fn create(&self, name: String) -> Result<Story> {
        tracing::debug!("create: name={}", name);

        let sql = "INSERT INTO stories (name) VALUES ($1) RETURNING id, name";
        let q = sqlx::query_as(sql);
        let story = q.bind(name).fetch_one(self.db_ref()).await?;

        Ok(story)
    }

    /// Update story name
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        tracing::debug!("update: id={}, name={}", id, name);

        let q = sqlx::query_as("UPDATE stories SET name = $1 WHERE id = $2 RETURNING id, name");
        let story = q.bind(name).bind(id).fetch_one(self.db_ref()).await?;

        Ok(story)
    }

    /// Delete a story and its tasks.
    pub async fn delete(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete: id={}", id);

        let mut tx = self.db.begin().await?;

        let q1 = sqlx::query("DELETE FROM tasks WHERE story_id = $1");
        let r1 = q1.bind(id).execute(&mut *tx).await?;

        let q2 = sqlx::query("DELETE FROM stories WHERE id = $1");
        let r2 = q2.bind(id).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(r1.rows_affected() + r2.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::tests;

    use testcontainers::{clients::Cli, RunnableImage};
    use testcontainers_modules::postgres::Postgres;

    #[ignore]
    #[tokio::test]
    async fn integration_test() {
        // Set up postgres test container backed repo
        let docker = Cli::default();
        let image = RunnableImage::from(Postgres::default()).with_tag("16-alpine");
        let container = docker.run(image);
        let pool = tests::setup_pg_pool(&container).await;

        // Set up repo under test
        let story_repo = StoryRepo::new(pool);

        // Create story
        let name = "Books To Read".to_string();
        let story = story_repo.create(name.clone()).await.unwrap();
        assert_eq!(name, story.name);

        // Query stories
        let stories = story_repo.list(std::i32::MAX).await.unwrap();
        assert_eq!(stories.len(), 1);

        // Delete the story
        let rows_updated = story_repo.delete(story.id).await.unwrap();
        assert_eq!(rows_updated, 1);

        // Assert story was deleted
        let stories = story_repo.list(std::i32::MAX).await.unwrap();
        assert!(stories.is_empty());
    }
}
