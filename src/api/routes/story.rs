use crate::{
    api::dto::page::{PageParams, PageToken},
    api::dto::story::{Stories, StoryBody},
    api::Ctx,
    domain::{Status, Story, Task},
    error::ErrorDto,
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
    paths(
        get_story,
        get_stories,
        get_tasks,
        create_story,
        update_story,
        delete_story,
    ),
    components(schemas(StoryBody, Story, Stories, Task, Status, ErrorDto)),
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
}

/// Get a story
#[utoipa::path(
    get,
    tag = "Story",
    path = "/stories/{id}",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 200, description = "Get a story by id", body = Story),
        (status = 404, description = "Story not found", body = ErrorDto)
    ),
)]
async fn get_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    let story = ctx.fetch_story(id).await?;
    Ok(Json(story))
}

/// Get a page of stories
#[utoipa::path(
    get,
    tag = "Story",
    path = "/stories",
    params(PageParams),
    responses(
        (status = 200, description = "Get a page of stories", body = Stories)
    )
)]
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("params: {:?}", params);
    let q = params.unwrap_or_default();
    let cursor = PageToken::decode_or(&q.page_token, 1)?;
    let (next_cursor, stories) = ctx.list_stories(cursor, q.page_size()).await?;
    let resp = Stories::new(PageToken::encode(next_cursor), stories);
    Ok(Json(resp))
}

/// Get all tasks for a story
#[utoipa::path(
    get,
    tag = "Story",
    path = "/stories/{id}/tasks",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 200, description = "Get tasks for a story", body = [Task])
    )
)]
async fn get_tasks(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let tasks = ctx.list_tasks(story_id).await?;
    Ok(Json(tasks))
}

/// Create a new story
#[utoipa::path(
    post,
    tag = "Story",
    path = "/stories",
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
    let story = ctx.create_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story
#[utoipa::path(
    patch,
    tag = "Story",
    path = "/stories/{id}",
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
        .fetch_story(id)
        .and_then(|_| ctx.update_story(id, name))
        .await;
    Ok(Json(story))
}

/// Delete a story
#[utoipa::path(
    delete,
    tag = "Story",
    path = "/stories/{id}",
    params(
        ("id" = Uuid, Path, description = "Story id")
    ),
    responses(
        (status = 204, description = "Story deleted"),
        (status = 404, description = "Story not found")
    )
)]
async fn delete_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    if let Err(err) = ctx.fetch_story(id).and_then(|_| ctx.delete_story(id)).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
