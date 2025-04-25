use crate::{
    action::story::DeleteStory,
    api::dto::{Page, PageParams, PageToken, StoryRequest, TaskParams},
    api::Ctx,
    domain::{Status, Story, Task},
    error::Errors,
    Result,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// OpenApi docs for story routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_story, get_stories, get_tasks, create_story, update_story, delete_story),
    components(schemas(Errors, Page<Story>, Page<Task>, Status, Story, StoryRequest, Task)),
    tags((name = "Story"))
)]
pub struct ApiDoc;

/// API routes for stories
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/{story_id}", get(get_story).delete(delete_story).patch(update_story))
        .route("/stories/{story_id}/tasks", get(get_tasks))
}

/// Get a story
#[utoipa::path(
    get,
    path = "/stories/{story_id}",
    params(("story_id" = Uuid, Path, description = "The story id")),
    responses(
        (status = 200, description = "The story", body = Story),
        (status = 404, description = "The story was not found", body = Errors)
    ),
    tag = "Story"
)]
async fn get_story(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let story = ctx.repo.fetch_story(story_id).await?;
    Ok(Json(story))
}

/// Get a page of stories
#[utoipa::path(
    get,
    path = "/stories",
    params(
        ("page_size" = Option<i32>,
            Query,
            minimum = 10,
            maximum = 1000,
            description = "The number of stories per page",
            nullable
        ),
        ("page_token" = Option<String>,
            Query,
            description = "The page cursor (next_page from response)",
            nullable
        )
    ),
    responses(
        (status = 200, description = "A page of stories", body = Page<Story>)
    ),
    tag = "Story"
)]
async fn get_stories(
    params: Query<PageParams>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("params: {:?}", params);
    let cursor = PageToken::decode_or(&params.page_token, 1)?;
    let limit = params.page_size();
    let (next_cursor, stories) = ctx.repo.list_stories(cursor, limit).await?;
    let next_page = PageToken::encode(next_cursor);
    Ok(Json(Page::new(next_page, stories)))
}

/// Get tasks for a story
#[utoipa::path(
    get,
    path = "/stories/{story_id}/tasks",
    params(
        ("story_id" = Uuid, Path, description = "The story id"),
        ("status" = Option<String>, Query, description = "The task status filter", nullable)
    ),
    responses(
        (status = 200, description = "The tasks for the story", body = Page<Task>),
        (status = 404, description = "The parent story was not found", body = Errors)
    ),
    tag = "Story"
)]
async fn get_tasks(
    params: Query<TaskParams>,
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let status = params.status();
    let mut tasks = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|s| ctx.repo.list_tasks(s.id))
        .await?;
    if let Some(status) = status {
        tasks.retain(|t| t.status == status.to_string());
    }
    Ok(Json(Page::new(None, tasks)))
}

/// Create a new story
#[utoipa::path(
    post,
    path = "/stories",
    request_body = StoryRequest,
    responses(
        (status = 201, description = "The story was created", body = Story),
        (status = 400, description = "The request body was invalid", body = Errors)
    ),
    tag = "Story"
)]
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(req): Json<StoryRequest>,
) -> Result<impl IntoResponse> {
    let name = req.validate()?;
    let story = ctx.repo.create_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story
#[utoipa::path(
    patch,
    path = "/stories/{story_id}",
    params(("story_id" = Uuid, Path, description = "The story id")),
    request_body = StoryRequest,
    responses(
        (status = 200, description = "The story was updated", body = Story),
        (status = 400, description = "The request body was invalid", body = Errors),
        (status = 404, description = "The story was not found", body = Errors)
    ),
    tag = "Story"
)]
async fn update_story(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(req): Json<StoryRequest>,
) -> Result<impl IntoResponse> {
    let name = req.validate()?;

    let story = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|s| ctx.repo.update_story(s.id, name))
        .await?;

    Ok(Json(story))
}

/// Delete a story
#[utoipa::path(
    delete,
    path = "/stories/{story_id}",
    params(("story_id" = Uuid, Path, description = "The story id")),
    responses(
        (status = 204, description = "The story was deleted"),
        (status = 404, description = "The story was not found")
    ),
    tag = "Story"
)]
async fn delete_story(Path(story_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    if let Err(err) = DeleteStory::execute(ctx, story_id).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
