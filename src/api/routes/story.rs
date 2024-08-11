use crate::{
    api::{
        dto::{
            page::{Page, PageParams, PageToken},
            story::StoryBody,
        },
        Ctx,
    },
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

/// API routes for stories
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id", get(get_story).delete(delete_story).patch(update_story))
        .route("/stories/:id/tasks", get(get_tasks))
}

/// Get story by id
async fn get_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    let story = ctx.fetch_story(id).await?;
    Ok(Json(story))
}

/// Get a page of stories
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::debug!("params: {:?}", params);
    let q = params.unwrap_or_default();
    let cursor = PageToken::decode_or(&q.page_token, 1)?;
    let (next_cursor, stories) = ctx.list_stories(cursor, q.page_size()).await?;
    let page = Page::new(PageToken::encode(next_cursor), stories);
    Ok(Json(page))
}

/// Get all tasks for a story
async fn get_tasks(
    Path(story_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    let tasks = ctx.list_tasks(story_id).await?;
    Ok(Json(tasks))
}

/// Create a new story for an owner
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    let name = body.validate()?;
    let story = ctx.create_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story name.
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

/// Delete a story by id
async fn delete_story(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    if let Err(err) = ctx.fetch_story(id).and_then(|_| ctx.delete_story(id)).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
