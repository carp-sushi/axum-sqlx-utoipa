use crate::{
    api::dto::{CreateTaskBody, PatchTaskBody},
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
    components(schemas(CreateTaskBody, Errors, PatchTaskBody, Status, Task)),
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
    tag = "Task",
    params(("task_id" = Uuid, Path, description = "Task id")),
    responses(
        (status = 200, description = "Get a task by id", body = Task),
        (status = 404, description = "Task not found", body = Errors)
    )
)]
async fn get_task(Path(task_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Task>> {
    let task = ctx.repo.fetch_task(task_id).await?;
    Ok(Json(task))
}

/// Create a task
#[utoipa::path(
    post,
    path = "/tasks",
    tag = "Task",
    request_body = CreateTaskBody,
    responses(
        (status = 201, description = "Task created", body = Task),
        (status = 400, description = "Invalid requesst body", body = Errors)
    )
)]
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    let (story_id, name, status) = body.validate()?;
    let task = ctx
        .repo
        .fetch_story(story_id)
        .and_then(|_| ctx.repo.create_task(story_id, name, status))
        .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task
#[utoipa::path(
    patch,
    path = "/tasks/{task_id}",
    tag = "Task",
    request_body = PatchTaskBody,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 400, description = "Invalid request body", body = Errors),
        (status = 404, description = "Task not found", body = Errors)
    )
)]
async fn update_task(
    Path(task_id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<Json<Task>> {
    let (name_opt, status_opt) = body.validate()?;
    let task = ctx
        .repo
        .fetch_task(task_id)
        .and_then(|task| {
            let name = name_opt.unwrap_or(task.name);
            let status = status_opt.unwrap_or(task.status);
            ctx.repo.update_task(task_id, name, status)
        })
        .await?;
    Ok(Json(task))
}

/// Delete a task
#[utoipa::path(
    delete,
    path = "/tasks/{task_id}",
    tag = "Task",
    params(("task_id" = Uuid, Path, description = "The task id")),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found")
    )
)]
async fn delete_task(Path(task_id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    let result = ctx
        .repo
        .fetch_task(task_id)
        .and_then(|_| ctx.repo.delete_task(task_id))
        .await;
    if let Err(err) = result {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}
