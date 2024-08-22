use crate::{api::Ctx, domain::StoryFile, error::ErrorDto, Error, Result};
use axum::{
    extract::{Multipart, Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// OpenApi docs for file routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_files, add_files, get_file, download_file),
    components(schemas(AddFile, ErrorDto, StoryFile)),
    tags((name = "File"))
)]
pub struct ApiDoc;

/// API routes for files
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories/:id/files", get(get_files).post(add_files))
        .route("/stories/:id/files/:file_id", get(get_file))
        .route("/stories/:id/files/:file_id/contents", get(download_file))
}

/// Get file metadata list for a story.
#[utoipa::path(
    get,
    path = "/stories/{id}/files",
    tag = "File",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 200, description = "Array of file metadata", body = [StoryFile]),
        (status = 404, description = "Story not found", body = ErrorDto)
    )
)]
async fn get_files(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let story_files = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|_| ctx.repo.list_files(story_id))
        .await?;
    Ok(Json(story_files))
}

// Just necessary for api docs
#[derive(ToSchema)]
struct AddFile {
    #[allow(dead_code)]
    file: Vec<u8>,
}

// Defaults for file uploads
const FILE: &str = "file.dat";
const OCTET: &str = "application/octet-stream";

/// Add files to a story.
#[utoipa::path(
    post,
    path = "/stories/{id}/files",
    tag = "File",
    params(
        ("id" = Uuid, Path, description = "Story id"),
    ),
    request_body(
        content_type = "multipart/form-data",
        content = AddFile,
    ),
    responses(
        (status = 200, description = "Array of file metadata", body = [StoryFile]),
        (status = 404, description = "Story not found", body = ErrorDto)
    )
)]
async fn add_files(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    mut form_data: Multipart,
) -> Result<impl IntoResponse> {
    let story = ctx.repo.fetch_story(id).await?;
    let mut files = Vec::new();
    while let Some(field) = form_data.next_field().await? {
        if field.name().unwrap_or_default() == "file" {
            let file_name = field.file_name().unwrap_or(FILE).to_string();
            let content_type = field.content_type().unwrap_or(OCTET).to_string();
            let bytes = field.bytes().await?;
            let storage_id = ctx.storage.write(&bytes).await?;
            let size = bytes.len() as i64;
            let file = ctx
                .repo
                .create_file(story.id, storage_id, file_name, size, content_type)
                .await?;
            files.push(file);
        }
    }
    if files.is_empty() {
        return Err(Error::invalid_args("no files uploaded"));
    }
    Ok(Json(files))
}

/// Download file contents.
#[utoipa::path(
    get,
    path = "/stories/{id}/files/{file_id}/contents",
    tag = "File",
    params(
        ("id" = Uuid, Path, description = "Story id"),
        ("file_id" = Uuid, Path, description = "File id")
    ),
    responses(
        (status = 200, description = "File contents"),
        (status = 404, description = "File not found", body = ErrorDto)
    )
)]
async fn download_file(
    Path((story_id, file_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let file = ctx.repo.fetch_file(story_id, file_id).await?;
    let contents = ctx.storage.read(file.storage_id).await?;
    let disposition = format!("attachment; filename=\"{}\"", file.name);
    let headers = [
        ("content-type", &file.content_type),
        ("content-disposition", &disposition),
    ];
    Ok((headers, contents).into_response())
}

/// Get file metadata.
#[utoipa::path(
    get,
    path = "/stories/{id}/files/{file_id}",
    tag = "File",
    params(
        ("id" = Uuid, Path, description = "Story id"),
        ("file_id" = Uuid, Path, description = "File id")
    ),
    responses(
        (status = 200, description = "File metadata", body = StoryFile),
        (status = 404, description = "File not found", body = ErrorDto)
    )
)]
async fn get_file(
    Path((story_id, file_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let file = ctx.repo.fetch_file(story_id, file_id).await?;
    Ok(Json(file))
}
