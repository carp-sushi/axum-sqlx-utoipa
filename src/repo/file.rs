use super::{sql, Repo};
use crate::{
    domain::{StorageId, StoryFile},
    Error, Result,
};
use futures_util::TryStreamExt;
use sqlx::{postgres::PgRow, FromRow, Row};
use stripmargin::StripMargin;
use uuid::Uuid;

// Defines a reasonable limit on the max files per story.
const MAX_FILES: i16 = 100;

/// Map sqlx rows to file metadata domain objects.
impl FromRow<'_, PgRow> for StoryFile {
    fn from_row(row: &PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            story_id: row.try_get("story_id")?,
            storage_id: StorageId(row.try_get("storage_id")?),
            name: row.try_get("name")?,
            size: row.try_get("size")?,
            content_type: row.try_get("content_type")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl Repo {
    /// Insert a new file metadata row.
    pub async fn create_file(
        &self,
        story_id: Uuid,
        StorageId(storage_id): StorageId,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile> {
        if size <= 0 {
            return Err(Error::invalid_args("file size must be > 0"));
        }
        let query = sql::file::CREATE.strip_margin();

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
        let query = sql::file::LIST.strip_margin();

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
        let query = sql::file::FETCH.strip_margin();

        let file_opt = sqlx::query_as(&query)
            .bind(file_id)
            .bind(story_id)
            .fetch_optional(self.db_ref())
            .await?;

        match file_opt {
            Some(file) => Ok(file),
            None => Err(Error::not_found(format!("file not found: {}", file_id))),
        }
    }

    /// Delete a file
    pub async fn delete_file(&self, id: Uuid) -> Result<u64> {
        let query = sql::file::DELETE.strip_margin();
        let result = sqlx::query(&query).bind(id).execute(self.db_ref()).await?;
        Ok(result.rows_affected())
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
        let name = "Project Requirements".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        assert_eq!(name, story.name);

        // Test file metadata
        let storage_id = StorageId::random();
        let name = "Sequence Diagrams.png".to_string();
        let size: i64 = 10420;
        let content_type = "image/png".to_string();

        // Add file
        let inserted = repo
            .create_file(story.id, storage_id.clone(), name, size, content_type)
            .await
            .unwrap();

        // Get file
        let file = repo.fetch_file(story.id, inserted.id).await.unwrap();
        assert_eq!(file.storage_id, storage_id);

        // List files
        let files = repo.list_files(story.id).await.unwrap();
        assert_eq!(files.len(), 1);
        assert!(files.contains(&file));

        // Delete file
        let rows = repo.delete_file(file.id).await.unwrap();
        assert!(rows > 0);
        let files = repo.list_files(story.id).await.unwrap();
        assert!(files.is_empty());
    }
}
