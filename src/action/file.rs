use crate::{api::Ctx, domain::StoryFile, Error, Result};
use axum::extract::Multipart;
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

// Defaults for file uploads
const FILE: &str = "file.dat";
const OCTET: &str = "application/octet-stream";

/// Store file contents and metadata from a multi-part form.
pub struct AddFiles;
impl AddFiles {
    pub async fn execute(
        ctx: Arc<Ctx>,
        story_id: Uuid,
        mut multipart: Multipart,
    ) -> Result<Vec<StoryFile>> {
        ctx.repo.fetch_story(story_id).await?;
        let mut files = Vec::new();
        while let Some(field) = multipart.next_field().await? {
            if field.name().unwrap_or_default() == "file" {
                let file_name = field.file_name().unwrap_or(FILE).to_string();
                let content_type = field.content_type().unwrap_or(OCTET).to_string();
                let bytes = field.bytes().await?;
                let storage_id = ctx.storage.write(&bytes).await?;
                let size = bytes.len() as i64;
                let file = ctx
                    .repo
                    .create_file(story_id, storage_id, file_name, size, content_type)
                    .await?;
                files.push(file);
            }
        }
        if files.is_empty() {
            return Err(Error::invalid_args("no files uploaded"));
        }
        Ok(files)
    }
}

/// Fetch file metadata and contents for download.
pub struct DownloadFile;
impl DownloadFile {
    pub async fn execute(
        ctx: Arc<Ctx>,
        story_id: Uuid,
        file_id: Uuid,
    ) -> Result<([(String, String); 2], Vec<u8>)> {
        let file = ctx
            .repo
            .fetch_story(story_id)
            .and_then(|s| ctx.repo.fetch_file(s.id, file_id))
            .await?;
        let contents = ctx.storage.read(file.storage_id).await?;
        let disposition = format!("attachment; filename=\"{}\"", file.name);
        let headers = [
            ("content-type".into(), file.content_type),
            ("content-disposition".into(), disposition),
        ];
        Ok((headers, contents))
    }
}

/// Delete file metadata, and purge contents from storage.
pub struct DeleteFile;
impl DeleteFile {
    pub async fn execute(ctx: Arc<Ctx>, story_id: Uuid, file_id: Uuid) -> Result<()> {
        // Delete file metadata
        let file = ctx
            .repo
            .fetch_story(story_id)
            .and_then(|story| ctx.repo.fetch_file(story.id, file_id))
            .and_then(|file| ctx.repo.delete_file(file))
            .await?;

        // Try to delete the file from storage, but only log error on failure
        if let Err(err) = ctx.storage.delete(file.storage_id).await {
            tracing::error!(
                "unable to delete file {} from storage: {}",
                file.storage_id,
                err
            );
        }

        Ok(())
    }
}
