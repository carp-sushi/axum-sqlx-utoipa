use super::{sql, Repo};
use crate::{
    domain::{Story, StoryFile},
    Error, Result,
};
use futures_util::TryStreamExt;
use sqlx::{postgres::PgRow, FromRow, Row};
use stripmargin::StripMargin;
use uuid::Uuid;

// Defines a reasonable limit on the max files per story.
const MAX_FILES: i16 = 100;

/// Map sqlx rows to story domain objects.
impl FromRow<'_, PgRow> for Story {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            seqno: row.try_get("seqno")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

/// Map sqlx rows to story file domain objects.
impl FromRow<'_, PgRow> for StoryFile {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            story_id: row.try_get("story_id")?,
            storage_id: row.try_get("storage_id")?,
            name: row.try_get("name")?,
            size: row.try_get("size")?,
            content_type: row.try_get("content_type")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

// Extend repo with queries related to stories.
impl Repo {
    /// Select a story by id
    pub async fn fetch_story(&self, id: Uuid) -> Result<Story> {
        let query = sql::story::FETCH.strip_margin();

        let story_opt = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(self.db_ref())
            .await?;

        match story_opt {
            Some(story) => Ok(story),
            None => Err(Error::not_found(format!("story not found: {id}"))),
        }
    }

    /// Select a page of stories.
    pub async fn list_stories(&self, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)> {
        let query = sql::story::LIST.strip_margin();

        let mut result_set = sqlx::query(&query)
            .bind(cursor)
            .bind(limit)
            .fetch(self.db_ref());

        let mut stories = Vec::with_capacity(limit as usize);

        while let Some(row) = result_set.try_next().await? {
            let story = Story::from_row(&row)?;
            stories.push(story);
        }

        let next_cursor = stories.last().map(|s| s.seqno + 1).unwrap_or_default();

        Ok((next_cursor, stories))
    }

    /// Insert a new story
    pub async fn create_story(&self, name: String) -> Result<Story> {
        let query = sql::story::CREATE.strip_margin();

        let story = sqlx::query_as(&query)
            .bind(name)
            .fetch_one(self.db_ref())
            .await?;

        Ok(story)
    }

    /// Update story name
    pub async fn update_story(&self, id: Uuid, name: String) -> Result<Story> {
        let query = sql::story::UPDATE.strip_margin();

        let story = sqlx::query_as(&query)
            .bind(name)
            .bind(id)
            .fetch_one(self.db_ref())
            .await?;

        Ok(story)
    }

    /// Delete a story and its tasks.
    pub async fn delete_story(&self, id: Uuid) -> Result<u64> {
        let mut tx = self.db.begin().await?;

        let query = sql::task::DELETE_BY_STORY.strip_margin();
        sqlx::query(&query).bind(id).execute(&mut *tx).await?;

        let query = sql::story::DELETE.strip_margin();
        let result = sqlx::query(&query).bind(id).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(result.rows_affected())
    }

    /// Insert a new story file
    pub async fn create_file(
        &self,
        story_id: Uuid,
        storage_id: Uuid,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile> {
        if size <= 0 {
            return Err(Error::invalid_args(&format!(
                "invalid file size: {}: size={}",
                name, size
            )));
        }

        let query = sql::story::ADD_FILE.strip_margin();

        let story_file = sqlx::query_as(&query)
            .bind(story_id)
            .bind(storage_id)
            .bind(name)
            .bind(size)
            .bind(content_type)
            .fetch_one(self.db_ref())
            .await?;

        Ok(story_file)
    }

    /// List all files for a story.
    pub async fn list_files(&self, story_id: Uuid) -> Result<Vec<StoryFile>> {
        let query = sql::story::LIST_FILES.strip_margin();

        let mut result_set = sqlx::query(&query)
            .bind(story_id)
            .bind(MAX_FILES)
            .fetch(self.db_ref());

        let mut story_files = Vec::with_capacity(MAX_FILES as usize);

        while let Some(row) = result_set.try_next().await? {
            let story_file = StoryFile::from_row(&row)?;
            story_files.push(story_file);
        }

        Ok(story_files)
    }

    /// Select a file by id and story id
    pub async fn fetch_file(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile> {
        let query = sql::story::FETCH_FILE.strip_margin();

        let file_opt = sqlx::query_as(&query)
            .bind(file_id)
            .bind(story_id)
            .fetch_optional(self.db_ref())
            .await?;

        match file_opt {
            Some(file) => Ok(file),
            None => Err(Error::not_found(format!("file not found: {file_id}"))),
        }
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
        let image = Postgres::default().with_tag("16-alpine");
        let container = image.start().await.unwrap();
        let pool = tests::setup_pg_pool(&container).await;
        let repo = Repo::new(pool);

        // Create story
        let name = "Books To Read".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        assert_eq!(name, story.name);

        // Query stories page
        let (_, stories) = repo.list_stories(1, 10).await.unwrap();
        assert_eq!(stories.len(), 1);

        // Update the name
        let updated_name = "Books".to_string();
        repo.update_story(story.id, updated_name).await.unwrap();

        // Fetch and verify new name
        let story = repo.fetch_story(story.id).await.unwrap();
        assert_eq!(story.name, "Books");

        // Delete the story
        let rows = repo.delete_story(story.id).await.unwrap();
        assert_eq!(rows, 1);

        // Assert story was deleted
        assert!(repo.fetch_story(story.id).await.is_err());
    }
}
