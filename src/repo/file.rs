use super::Repo;
use crate::{
    domain::{StorageId, StoryFile, StoryFileId, StoryId},
    Error, Result,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Defines a reasonable limit on the max files per story.
const MAX_FILES: i16 = 100;

/// The task entity object - used for query validation against the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StoryFileEntity {
    pub id: Uuid,
    pub story_id: Uuid,
    pub storage_id: Uuid,
    pub name: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// The repo should map the entity to the domain object in public functions.
impl From<StoryFileEntity> for StoryFile {
    fn from(entity: StoryFileEntity) -> Self {
        Self {
            id: StoryFileId(entity.id),
            story_id: StoryId(entity.story_id),
            storage_id: StorageId(entity.storage_id),
            name: entity.name,
            size: entity.size,
            content_type: entity.content_type,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl Repo {
    /// Insert a new file metadata row.
    pub async fn create_file(
        &self,
        &StoryId(story_id): &StoryId,
        &StorageId(storage_id): &StorageId,
        name: String,
        size: i64,
        content_type: String,
    ) -> Result<StoryFile> {
        if size <= 0 {
            return Err(Error::invalid_args("file size must be > 0"));
        }
        let query = sqlx::query_as!(
            StoryFileEntity,
            r#"INSERT INTO story_files (story_id, storage_id, name, size, content_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, story_id, storage_id, name, size, content_type, created_at, updated_at"#,
            story_id,
            storage_id,
            name,
            size,
            content_type,
        );
        let entity = query.fetch_one(self.db_ref()).await?;
        Ok(StoryFile::from(entity))
    }

    /// List all files for a story.
    pub async fn list_files(&self, &StoryId(story_id): &StoryId) -> Result<Vec<StoryFile>> {
        let query = sqlx::query_as!(
            StoryFileEntity,
            r#"SELECT id, story_id, storage_id, name, size, content_type, created_at, updated_at
            FROM story_files WHERE story_id = $1
            ORDER BY created_at LIMIT $2"#,
            story_id,
            MAX_FILES as i64,
        );
        let story_files = query.fetch_all(self.db_ref()).await?;
        Ok(story_files.into_iter().map(StoryFile::from).collect())
    }

    /// Select a file by id and story id
    pub async fn fetch_file(
        &self,
        &StoryId(story_id): &StoryId,
        &StoryFileId(file_id): &StoryFileId,
    ) -> Result<StoryFile> {
        let query = sqlx::query_as!(
            StoryFileEntity,
            r#"SELECT id, story_id, storage_id, name, size, content_type, created_at, updated_at
            FROM story_files WHERE id = $1 AND story_id = $2"#,
            file_id,
            story_id,
        );
        match query.fetch_optional(self.db_ref()).await? {
            Some(entity) => Ok(StoryFile::from(entity)),
            None => Err(Error::not_found(format!("file not found: {file_id}"))),
        }
    }

    /// Delete a file
    pub async fn delete_file(&self, file: StoryFile) -> Result<StoryFile> {
        let StoryFileId(file_id) = file.id;
        sqlx::query!("DELETE FROM story_files WHERE id = $1", file_id)
            .execute(self.db_ref())
            .await?;
        Ok(file)
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
        let name = "Project Requirements".to_string();
        let story = repo.create_story(name.clone()).await.unwrap();
        assert_eq!(name, story.name);

        // Test file metadata
        let storage_id: StorageId = StorageId(Uuid::new_v4());
        let name = "Sequence Diagrams.png".to_string();
        let size: i64 = 10420;
        let content_type = "image/png".to_string();

        // Add file
        let inserted = repo
            .create_file(&story.id, &storage_id, name, size, content_type)
            .await
            .unwrap();

        // Get file
        let file = repo.fetch_file(&story.id, &inserted.id).await.unwrap();
        assert_eq!(file.storage_id, storage_id);

        // List files
        let files = repo.list_files(&story.id).await.unwrap();
        assert_eq!(files.len(), 1);
        assert!(files.contains(&file));

        // Delete file
        repo.delete_file(file).await.unwrap();
        let files = repo.list_files(&story.id).await.unwrap();
        assert!(files.is_empty());

        // Cleanup
        repo.delete_story(&story.id).await.unwrap();
    }
}
