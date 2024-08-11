use axum::{routing::get, Json, Router};
use std::sync::Arc;
use utoipa::{openapi::OpenApi as OpenApiResp, OpenApi};

mod ctx;
mod dto;
pub mod routes;
mod tracer;

pub use ctx::Ctx;

use routes::{status, story, task};

/// The top-level API
pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    /// Create a new api
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    /// Define API routes, mapping paths to handlers.
    pub fn routes(self) -> Router {
        tracer::wrap(
            Router::new()
                .route("/openapi.json", get(openapi))
                .merge(status::routes())
                .merge(story::routes())
                .merge(task::routes()),
        )
        .with_state(self.ctx)
    }
}

/// Combine and serve openapi docs for internal routes.
async fn openapi() -> Json<OpenApiResp> {
    let mut api = story::ApiDoc::openapi();
    api.merge(task::ApiDoc::openapi());
    Json(api)
}
