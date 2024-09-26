use crate::{api::Ctx, domain::Story, Result};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// Fetch a story
pub struct GetStory;
impl GetStory {
    pub async fn execute(ctx: Arc<Ctx>, story_id: Uuid) -> Result<Story> {
        ctx.repo.fetch_story(story_id).await
    }
}

/// Fetch a page of stories
pub struct GetStories;
impl GetStories {
    pub async fn execute(ctx: Arc<Ctx>, cursor: i64, limit: i32) -> Result<(i64, Vec<Story>)> {
        ctx.repo.list_stories(cursor, limit).await
    }
}

/// Create a story
pub struct CreateStory;
impl CreateStory {
    pub async fn execute(ctx: Arc<Ctx>, name: String) -> Result<Story> {
        ctx.repo.create_story(name).await
    }
}

/// Update a story
pub struct UpdateStory;
impl UpdateStory {
    pub async fn execute(ctx: Arc<Ctx>, story_id: Uuid, name: String) -> Result<Story> {
        let story = ctx
            .repo
            .fetch_story(story_id)
            .and_then(|s| ctx.repo.update_story(s.id, name))
            .await?;
        Ok(story)
    }
}

/// Delete a story
pub struct DeleteStory;
impl DeleteStory {
    pub async fn execute(ctx: Arc<Ctx>, story_id: Uuid) -> Result<()> {
        // Ensure story exists
        ctx.repo.fetch_story(story_id).await?;

        // Gather all storage references
        let mut storage_ids = Vec::new();
        if let Ok(files) = ctx.repo.list_files(story_id).await {
            storage_ids.extend(files.into_iter().map(|f| f.storage_id));
        }

        // Delete all story metadata
        ctx.repo.delete_story(story_id).await?;

        // Delete file contents from storage only after metadata deletion succeeds
        for storage_id in storage_ids {
            // Don't fail action, just log the error
            if let Err(err) = ctx.storage.delete(storage_id).await {
                tracing::error!("unable to delete {} from storage: {}", storage_id, err);
            }
        }

        Ok(())
    }
}
