use super::Repo;
use crate::{
    domain::{Story, StoryId},
    Error, Result,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The story entity object - used for query validation against the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StoryEntity {
    id: Uuid,
    name: String,
    seqno: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// The repo should map the entity to the domain object in public functions.
impl From<StoryEntity> for Story {
    fn from(entity: StoryEntity) -> Self {
        Self {
            id: StoryId(entity.id),
            name: entity.name,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

// Extend repo with queries related to stories.
impl Repo {
    /// Select a story by id
    pub async fn fetch_story(&self, &StoryId(story_id): &StoryId) -> Result<Story> {
        let query = sqlx::query_as!(
            StoryEntity,
            "SELECT id, name, seqno, created_at, updated_at FROM stories WHERE id = $1",
            story_id
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(entity) => Ok(Story::from(entity)),
            None => Err(Error::not_found(format!("story not found: {story_id}"))),
        }
    }

    /// Select a page of stories.
    pub async fn list_stories(&self, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)> {
        let query = sqlx::query_as!(
            StoryEntity,
            r#"SELECT id, name, seqno, created_at, updated_at FROM stories WHERE seqno >= $1
            ORDER BY seqno LIMIT $2"#,
            cursor,
            limit as i64,
        );
        let entities = query.fetch_all(self.db_ref()).await?;
        let next_cursor = entities.last().map(|s| s.seqno + 1).unwrap_or_default();
        let stories = entities.into_iter().map(Story::from).collect();
        Ok((next_cursor, stories))
    }

    /// Insert a new story
    pub async fn create_story(&self, name: impl Into<String>) -> Result<Story> {
        let query = sqlx::query_as!(
            StoryEntity,
            r#"INSERT INTO stories (name) VALUES ($1)
            RETURNING id, name, seqno, created_at, updated_at"#,
            name.into()
        );
        let entity = query.fetch_one(self.db_ref()).await?;
        Ok(Story::from(entity))
    }

    /// Update story name
    pub async fn update_story(
        &self,
        &StoryId(story_id): &StoryId,
        name: impl Into<String>,
    ) -> Result<Story> {
        let query = sqlx::query_as!(
            StoryEntity,
            r#"UPDATE stories SET name = $1 WHERE id = $2
            RETURNING id, name, seqno, created_at, updated_at"#,
            name.into(),
            story_id
        );
        let entity = query.fetch_one(self.db_ref()).await?;
        Ok(Story::from(entity))
    }

    /// Delete a story, child files, and child tasks.
    pub async fn delete_story(&self, &StoryId(story_id): &StoryId) -> Result<()> {
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
        assert_eq!(story.name, "Books To Read");

        // Query stories page
        let (next_cursor, stories) = repo.list_stories(1, 10).await.unwrap();
        assert_eq!(next_cursor, 2);
        assert_eq!(stories.len(), 1);

        // Update the story name
        repo.update_story(&story.id, "Books").await.unwrap();
        let story = repo.fetch_story(&story.id).await.unwrap();
        assert_eq!(story.name, "Books");

        // Delete the story
        repo.delete_story(&story.id).await.unwrap();
        assert!(repo.fetch_story(&story.id).await.is_err());
    }
}
