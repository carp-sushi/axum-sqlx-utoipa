use super::Repo;
use crate::{domain::Story, Error, Result};
use uuid::Uuid;

// Extend repo with queries related to stories.
impl Repo {
    /// Select a story by id
    pub async fn fetch_story(&self, story_id: Uuid) -> Result<Story> {
        let query = sqlx::query_as!(
            Story,
            "SELECT id, name, seqno, created_at, updated_at FROM stories WHERE id = $1",
            story_id
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(story) => Ok(story),
            None => Err(Error::not_found(format!("story not found: {story_id}"))),
        }
    }

    /// Select a page of stories.
    pub async fn list_stories(&self, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)> {
        let query = sqlx::query_as!(
            Story,
            r#"SELECT id, name, seqno, created_at, updated_at FROM stories WHERE seqno >= $1
            ORDER BY seqno LIMIT $2"#,
            cursor,
            limit as i64,
        );
        let stories = query.fetch_all(self.db_ref()).await?;
        let next_cursor = stories.last().map(|s| s.seqno + 1).unwrap_or_default();
        Ok((next_cursor, stories))
    }

    /// Insert a new story
    pub async fn create_story(&self, name: impl Into<String>) -> Result<Story> {
        let query = sqlx::query_as!(
            Story,
            r#"INSERT INTO stories (name) VALUES ($1)
            RETURNING id, name, seqno, created_at, updated_at"#,
            name.into()
        );
        let story = query.fetch_one(self.db_ref()).await?;
        Ok(story)
    }

    /// Update story name
    pub async fn update_story(&self, story_id: Uuid, name: impl Into<String>) -> Result<Story> {
        let query = sqlx::query_as!(
            Story,
            r#"UPDATE stories SET name = $1, updated_at = now() WHERE id = $2
            RETURNING id, name, seqno, created_at, updated_at"#,
            name.into(),
            story_id
        );
        let story = query.fetch_one(self.db_ref()).await?;
        Ok(story)
    }

    /// Delete a story, child files, and child tasks.
    pub async fn delete_story(&self, story_id: Uuid) -> Result<()> {
        let mut tx = self.db.begin().await?;

        sqlx::query!("DELETE FROM tasks WHERE story_id = $1", story_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM story_files WHERE story_id = $1", story_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM stories WHERE id = $1", story_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::tests;

    use testcontainers::{runners::AsyncRunner, ImageExt};
    use testcontainers_modules::postgres::Postgres;

    #[ignore]
    #[tokio::test]
    async fn integration_test() {
        // Set up postgres test container backed repo
        let image = Postgres::default().with_tag(tests::PG_VERSION_TAG);
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(pool);

        // Create story
        let story = repo.create_story("Books To Read").await.unwrap();
        assert_eq!("Books To Read", story.name);

        // Query stories page
        let (_, stories) = repo.list_stories(1, 10).await.unwrap();
        assert_eq!(stories.len(), 1);

        // Update the name
        repo.update_story(story.id, "Books").await.unwrap();

        // Fetch and verify new name
        let story = repo.fetch_story(story.id).await.unwrap();
        assert_eq!(story.name, "Books");

        // Delete the story
        repo.delete_story(story.id).await.unwrap();

        // Assert story was deleted
        assert!(repo.fetch_story(story.id).await.is_err());
    }
}
