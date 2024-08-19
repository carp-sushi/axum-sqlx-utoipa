use crate::{
    api::dto::{PageParams, PageToken, Stories, StoryBody, TaskParams},
    api::Ctx,
    domain::{Status, Story, StoryFile, Task},
    error::ErrorDto,
    Result,
};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// OpenApi docs for story routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        get_story, get_stories, get_tasks, create_story, update_story, delete_story,
        get_files, add_files, get_file, download_file
    ),
    components(
        schemas(AddFile, ErrorDto, Status, Stories, Story, StoryBody, StoryFile, Task)
    ),
    tags((name = "Story"))
)]
pub struct ApiDoc;

/// API routes for stories
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id", get(get_story).delete(delete_story).patch(update_story))
        .route("/stories/:id/tasks", get(get_tasks))
        .route("/stories/:id/files", get(get_files).post(add_files))
        .route("/stories/:id/files/:file_id", get(get_file))
        .route("/stories/:id/files/:file_id/contents", get(download_file))
}

/// Get a story
#[utoipa::path(
    get,
    path = "/stories/{id}",
    tag = "Story",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 200, description = "The story", body = Story),
        (status = 404, description = "Story not found", body = ErrorDto)
    )
)]
async fn get_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    let story = ctx.repo.fetch_story(id).await?;
    Ok(Json(story))
}

/// Get a page of stories
#[utoipa::path(
    get,
    path = "/stories",
    tag = "Story",
    params(
        ("page_size" = Option<i32>,
            Query,
            minimum = 10,
            maximum = 1000,
            description = "Number of stories per page",
            nullable
        ),
        ("page_token" = Option<String>,
            Query,
            description = "Page cursor (next_page from response)",
            nullable
        )
    ),
    responses(
        (status = 200, description = "A page of stories", body = Stories)
    )
)]
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("params: {:?}", params);
    let q = params.unwrap_or_default();
    let cursor = PageToken::decode_or(&q.page_token, 1)?;
    let (next_cursor, stories) = ctx.repo.list_stories(cursor, q.page_size()).await?;
    let resp = Stories::new(PageToken::encode(next_cursor), stories);
    Ok(Json(resp))
}

/// Get tasks for a story
#[utoipa::path(
    get,
    path = "/stories/{id}/tasks",
    tag = "Story",
    params(
        ("id" = Uuid, Path, description = "Story id"),
        ("status" = Option<String>, Query, description = "Task status", nullable)
    ),
    responses(
        (status = 200, description = "An array of tasks", body = [Task])
    )
)]
async fn get_tasks(
    params: Option<Query<TaskParams>>,
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("params: {:?}", params);
    let q = params.unwrap_or_default();
    let mut tasks = ctx.repo.list_tasks(story_id).await?;
    if let Some(status) = q.status() {
        tasks.retain(|t| t.status == status);
    }
    Ok(Json(tasks))
}

/// Create a new story
#[utoipa::path(
    post,
    path = "/stories",
    tag = "Story",
    request_body = StoryBody,
    responses(
        (status = 201, description = "Story created", body = Story),
        (status = 400, description = "Invalid request body", body = ErrorDto)
    )
)]
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    let name = body.validate()?;
    let story = ctx.repo.create_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story
#[utoipa::path(
    patch,
    path = "/stories/{id}",
    tag = "Story",
    request_body = StoryBody,
    responses(
        (status = 200, description = "Story updated", body = Story),
        (status = 400, description = "Invalid request body", body = ErrorDto),
        (status = 404, description = "Story not found", body = ErrorDto)
    )
)]
async fn update_story(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    let name = body.validate()?;
    let story = ctx
        .repo
        .fetch_story(id)
        .and_then(|_| ctx.repo.update_story(id, name))
        .await?;
    Ok(Json(story))
}

/// Delete a story
#[utoipa::path(
    delete,
    path = "/stories/{id}",
    tag = "Story",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 204, description = "Story deleted"),
        (status = 404, description = "Story not found")
    )
)]
async fn delete_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    let result = ctx
        .repo
        .fetch_story(id)
        .and_then(|_| ctx.repo.delete_story(id))
        .await;
    if let Err(err) = result {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}

/// Get file metadata list for a story.
#[utoipa::path(
    get,
    path = "/stories/{id}/files",
    tag = "Story",
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

/// Add files to a story.
#[utoipa::path(
    post,
    path = "/stories/{id}/files",
    tag = "Story",
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
            let file_name = field.file_name().unwrap_or_default().to_string();
            let content_type = field.content_type().unwrap_or_default().to_string();
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
    Ok(Json(files))
}

/// Download file contents.
#[utoipa::path(
    get,
    path = "/stories/{id}/files/{file_id}/contents",
    tag = "Story",
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
    tag = "Story",
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
