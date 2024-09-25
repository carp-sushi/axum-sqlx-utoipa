use crate::{
    api::dto::{CreateTaskRequest, UpdateTaskRequest},
    api::Ctx,
    domain::{Status, Task},
    error::Errors,
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

/// OpenApi docs for story routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_task, create_task, update_task, delete_task),
    components(schemas(CreateTaskRequest, Errors, Status, Task, UpdateTaskRequest)),
    tags((name = "Task"))
)]
pub struct ApiDoc;

/// API routes for tasks
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:task_id", get(get_task).delete(delete_task).patch(update_task))
}

/// Get a task
#[utoipa::path(
    get,
    path = "/tasks/{task_id}",
    params(("task_id" = Uuid, Path, description = "The task id")),
    responses(
        (status = 200, description = "The task", body = Task),
        (status = 404, description = "The task was not found", body = Errors)
    ),
    tag = "Task"
)]
async fn get_task(Path(task_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Task>> {
    let task = ctx.repo.fetch_task(task_id).await?;
    Ok(Json(task))
}

/// Create a task
#[utoipa::path(
    post,
    path = "/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "The task was created", body = Task),
        (status = 400, description = "The requesst body was invalid", body = Errors)
    ),
    tag = "Task"
)]
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse> {
    let (story_id, name, status) = req.validate()?;
    let task = ctx.repo.create_task(story_id, name, status).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task
#[utoipa::path(
    patch,
    path = "/tasks/{task_id}",
    params(("task_id" = Uuid, Path, description = "The task id")),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "The task was updated", body = Task),
        (status = 400, description = "The request body was invalid", body = Errors),
        (status = 404, description = "The task was not found", body = Errors)
    ),
    tag = "Task"
)]
async fn update_task(
    Path(task_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<Task>> {
    let (name, status) = req.validate()?;
    let task = ctx
        .repo
        .fetch_task(task_id)
        .and_then(|t| {
            let status = status.unwrap_or(t.status());
            let name = name.unwrap_or(t.name);
            ctx.repo.update_task(task_id, name, status)
        })
        .await?;
    Ok(Json(task))
}

/// Delete a task
#[utoipa::path(
    delete,
    path = "/tasks/{task_id}",
    params(("task_id" = Uuid, Path, description = "The task id")),
    responses(
        (status = 204, description = "The task was deleted"),
        (status = 404, description = "The task was not found")
    ),
    tag = "Task"
)]
async fn delete_task(Path(task_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    let result = ctx
        .repo
        .fetch_task(task_id)
        .and_then(|t| ctx.repo.delete_task(t.id))
        .await;
    if let Err(err) = result {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
