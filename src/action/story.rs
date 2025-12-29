use crate::{api::Ctx, Result};
use std::sync::Arc;
use uuid::Uuid;

/// Delete a story
pub struct DeleteStory;
impl DeleteStory {
    pub async fn execute(ctx: Arc<Ctx>, story_id: Uuid) -> Result<()> {
        // Ensure story exists
        ctx.repo.fetch_story(story_id).await?;

        // Gather all storage references
        let storage_ids = if let Ok(files) = ctx.repo.list_files(story_id).await {
            files.into_iter().map(|f| f.storage_id).collect()
        } else {
            vec![]
        };

        // Delete story, tasks, and file metadata
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
