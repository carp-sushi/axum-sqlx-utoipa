use crate::{
    api::{dto::StoryBody, Ctx},
    domain::{Story, Task},
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use validator::Validate;

/// API routes for stories
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id/tasks", get(get_tasks))
        .route(
            "/stories/:id",
            get(get_story).delete(delete_story).patch(update_story),
        )
}

/// Get story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Story>> {
    tracing::info!("GET /stories/{}", id);
    let story = ctx.story.get(id).await?;
    Ok(Json(story))
}

/// Get stories by owner
async fn get_stories(State(ctx): State<Arc<Ctx>>) -> Result<Json<Vec<Story>>> {
    tracing::info!("GET /stories");
    let stories = ctx.story.list().await?;
    Ok(Json(stories))
}

/// Get tasks for a story
async fn get_tasks(
    Path(story_id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<Json<Vec<Task>>> {
    tracing::info!("GET /stories/{}/tasks", story_id);
    let tasks = ctx.task.list(story_id).await?;
    Ok(Json(tasks))
}

/// Create a new story for an owner
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /stories");
    body.validate()?;
    let story = ctx.story.create(body.name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story name.
async fn update_story(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<Json<Story>> {
    tracing::info!("PATCH /stories/{}", id);
    body.validate()?;
    let story = ctx.story.update(id, body.name).await?;
    Ok(Json(story))
}

/// Delete a story by id
async fn delete_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /stories/{}", id);
    match ctx.story.delete(id).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(err) => StatusCode::from(err),
    }
}
