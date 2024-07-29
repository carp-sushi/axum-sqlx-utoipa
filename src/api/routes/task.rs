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
use std::sync::Arc;

/// API routes for tasks
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task).delete(delete_task).patch(update_task))
}

/// Get task by id
async fn get_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Task>> {
    tracing::info!("GET /tasks/{}", id);
    let task = ctx.tasks.get(id).await?;
    Ok(Json(task))
}

/// Create a task new task
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /tasks");
    let (story_id, name) = body.validate()?;
    let task = ctx.tasks.create(story_id, name).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task name and/or status.
async fn update_task(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<Json<Task>> {
    tracing::info!("PATCH /tasks/{}", id);
    let (name, status) = body.validate()?;
    let task = ctx.tasks.update(id, name, status).await?;
    Ok(Json(task))
}

/// Delete a task by id
async fn delete_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /tasks/{}", id);
    if let Err(err) = ctx.tasks.delete(id).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
