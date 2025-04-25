use crate::{
    action::file::{AddFiles, DeleteFile, DownloadFile},
    api::dto::Page,
    api::Ctx,
    domain::StoryFile,
    error::Errors,
    Result,
};
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

// Just necessary for api docs
#[derive(ToSchema)]
struct FileUpload {
    #[allow(dead_code)]
    file: Vec<u8>,
}

/// OpenApi docs for file routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_files, add_files, get_file, download_file, delete_file),
    components(schemas(Errors, FileUpload, Page<StoryFile>, StoryFile)),
    tags((name = "File"))
)]
pub struct ApiDoc;

/// API routes for files
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories/{story_id}/files", get(get_files).post(add_files))
        .route("/stories/{story_id}/files/{file_id}", get(get_file).delete(delete_file))
        .route("/stories/{story_id}/files/{file_id}/contents", get(download_file))
}

/// List files for a story.
#[utoipa::path(
    get,
    path = "/stories/{story_id}/files",
    params(("story_id" = Uuid, Path, description = "The parent story id")),
    responses(
        (status = 200, description = "A file metadata array for the story", body = Page<StoryFile>),
        (status = 404, description = "The parent story was not found", body = Errors)
    ),
    tag = "File"
)]
async fn get_files(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let files = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|s| ctx.repo.list_files(s.id))
        .await?;
    Ok(Json(Page::new(None, files)))
}

/// Add files to a story.
#[utoipa::path(
    post,
    path = "/stories/{story_id}/files",
    params(("story_id" = Uuid, Path, description = "The parent story id")),
    request_body(
        content_type = "multipart/form-data",
        content = FileUpload,
    ),
    responses(
        (status = 201, description = "A metadata array for the uploaded files", body = [StoryFile]),
        (status = 404, description = "The parent story was not found", body = Errors)
    ),
    tag = "File"
)]
async fn add_files(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    multipart: Multipart,
) -> Result<impl IntoResponse> {
    let files = AddFiles::execute(ctx, story_id, multipart).await?;
    Ok((StatusCode::CREATED, Json(files)))
}

/// Download file contents.
#[utoipa::path(
    get,
    path = "/stories/{story_id}/files/{file_id}/contents",
    params(
        ("story_id" = Uuid, Path, description = "The parent story id"),
        ("file_id" = Uuid, Path, description = "The id of the file to download")
    ),
    responses(
        (status = 200, description = "The contents of the file"),
        (status = 404, description = "The file was not found", body = Errors)
    ),
    tag = "File"
)]
async fn download_file(
    Path((story_id, file_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let (headers, contents) = DownloadFile::execute(ctx, story_id, file_id).await?;
    Ok((headers, contents).into_response())
}

/// Get file metadata.
#[utoipa::path(
    get,
    path = "/stories/{story_id}/files/{file_id}",
    params(
        ("story_id" = Uuid, Path, description = "The parent story id"),
        ("file_id" = Uuid, Path, description = "The id of the file metadata")
    ),
    responses(
        (status = 200, description = "The file metadata", body = StoryFile),
        (status = 404, description = "The file was not found", body = Errors)
    ),
    tag = "File"
)]
async fn get_file(
    Path((story_id, file_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let file = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|s| ctx.repo.fetch_file(s.id, file_id))
        .await?;
    Ok(Json(file))
}

/// Delete a file
#[utoipa::path(
    delete,
    path = "/stories/{story_id}/files/{file_id}",
    params(
        ("story_id" = Uuid, Path, description = "The parent story id"),
        ("file_id" = Uuid, Path, description = "The id of the file to delete")
    ),
    responses(
        (status = 204, description = "The file was deleted successfully"),
        (status = 404, description = "The file was not found")
    ),
    tag = "File"
)]
async fn delete_file(
    Path((story_id, file_id)): Path<(Uuid, Uuid)>,
    State(ctx): State<Arc<Ctx>>,
) -> StatusCode {
    if let Err(err) = DeleteFile::execute(ctx, story_id, file_id).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
