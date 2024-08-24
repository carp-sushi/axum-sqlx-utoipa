use crate::{
    api::dto::{PageParams, PageToken, Stories, StoryBody, TaskParams},
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
    components(schemas(Errors, Status, Stories, Story, StoryBody, Task)),
    tags((name = "Story"))
)]
pub struct ApiDoc;

/// API routes for stories
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:story_id", get(get_story).delete(delete_story).patch(update_story))
        .route("/stories/:story_id/tasks", get(get_tasks))
}

/// Get a story
#[utoipa::path(
    get,
    path = "/stories/{story_id}",
    tag = "Story",
    params(("story_id" = Uuid, Path, description = "Story id")),
    responses(
        (status = 200, description = "The story", body = Story),
        (status = 404, description = "Story not found", body = Errors)
    )
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
    path = "/stories/{story_id}/tasks",
    tag = "Story",
    params(
        ("story_id" = Uuid, Path, description = "Story id"),
        ("status" = Option<String>, Query, description = "Task status", nullable)
    ),
    responses((status = 200, description = "An array of tasks", body = [Task]))
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
        (status = 400, description = "Invalid request body", body = Errors)
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
    path = "/stories/{story_id}",
    tag = "Story",
    request_body = StoryBody,
    responses(
        (status = 200, description = "Story updated", body = Story),
        (status = 400, description = "Invalid request body", body = Errors),
        (status = 404, description = "Story not found", body = Errors)
    )
)]
async fn update_story(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    let name = body.validate()?;
    let story = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|_| ctx.repo.update_story(story_id, name))
        .await?;
    Ok(Json(story))
}

/// Delete a story
#[utoipa::path(
    delete,
    path = "/stories/{story_id}",
    tag = "Story",
    params(("story_id" = Uuid, Path, description = "Story id")),
    responses(
        (status = 204, description = "Story deleted"),
        (status = 404, description = "Story not found")
    )
)]
async fn delete_story(Path(story_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    let result = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|_| ctx.repo.delete_story(story_id))
        .await;
    if let Err(err) = result {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
