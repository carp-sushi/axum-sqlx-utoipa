use crate::{
    api::dto::task::{CreateTaskBody, PatchTaskBody},
    api::Ctx,
    domain::Task,
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// API routes for tasks
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task).delete(delete_task).patch(update_task))
}

/// Get task by id
async fn get_task(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Task>> {
    let task = ctx.fetch_task(id).await?;
    Ok(Json(task))
}

/// Create a task new task
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    let (story_id, name, status) = body.validate()?;
    let task = ctx
        .fetch_story(story_id)
        .and_then(|_| ctx.create_task(story_id, name, status))
        .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task name and/or status.
async fn update_task(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<Json<Task>> {
    let (name_opt, status_opt) = body.validate()?;
    let task = ctx
        .fetch_task(id)
        .and_then(|task| {
            let name = name_opt.unwrap_or(task.name);
            let status = status_opt.unwrap_or(task.status);
            ctx.update_task(id, name, status)
        })
        .await?;
    Ok(Json(task))
}

/// Delete a task by id
async fn delete_task(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    if let Err(err) = ctx.fetch_task(id).and_then(|_| ctx.delete_task(id)).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
