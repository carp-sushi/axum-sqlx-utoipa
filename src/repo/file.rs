use super::Repo;
use crate::{domain::StoryFile, Error, Result};
use uuid::Uuid;

// Defines a reasonable limit on the max files per story.
const MAX_FILES: i16 = 100;

impl Repo {
    /// Insert a new file metadata row.
    pub async fn create_file(
        &self,
        story_id: Uuid,
        storage_id: Uuid,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile> {
        if size <= 0 {
            return Err(Error::invalid_args("file size must be > 0"));
        }
        let query = sqlx::query_as!(
            StoryFile,
            r#"INSERT INTO story_files (story_id, storage_id, name, size, content_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, story_id, storage_id, name, size, content_type, created_at, updated_at"#,
            story_id,
            storage_id,
            name,
            size,
            content_type,
        );
        let story_file = query.fetch_one(self.db_ref()).await?;
        Ok(story_file)
    }

    /// List all files for a story.
    pub async fn list_files(&self, story_id: Uuid) -> Result<Vec<StoryFile>> {
        let query = sqlx::query_as!(
            StoryFile,
            r#"SELECT id, story_id, storage_id, name, size, content_type, created_at, updated_at
            FROM story_files WHERE story_id = $1
            ORDER BY created_at LIMIT $2"#,
            story_id,
            MAX_FILES as i64,
        );
        let story_files = query.fetch_all(self.db_ref()).await?;
        Ok(story_files)
    }

    /// Select a file by id and story id
    pub async fn fetch_file(&self, story_id: Uuid, file_id: Uuid) -> Result<StoryFile> {
        let query = sqlx::query_as!(
            StoryFile,
            r#"SELECT id, story_id, storage_id, name, size, content_type, created_at, updated_at
            FROM story_files WHERE id = $1 AND story_id = $2"#,
            file_id,
            story_id,
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(file) => Ok(file),
            None => Err(Error::not_found(format!("file not found: {}", file_id))),
        }
    }

    /// Delete a file
    pub async fn delete_file(&self, id: Uuid) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM story_files WHERE id = $1", id)
            .execute(self.db_ref())
            .await?;
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
        let storage_id = Uuid::new_v4();
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
