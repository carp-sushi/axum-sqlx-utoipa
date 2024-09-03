use axum::Router;
use std::sync::Arc;
use utoipa::{openapi::OpenApi as OpenApiDocs, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod ctx;
pub use ctx::Ctx;
mod dto;
mod routes;
use routes::{file, status, story, task};
mod tracer;

/// The top-level API
pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    /// Create a new api with context pointer state.
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    /// Create an API service by merging internal routes with context pointer state.
    pub fn mk_service(self) -> Router {
        tracer::wrap(
            Router::new()
                .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs()))
                .merge(status::routes())
                .merge(story::routes())
                .merge(file::routes())
                .merge(task::routes()),
        )
        .with_state(self.ctx)
    }
}

/// Combined OpenApi docs of internal routes.
pub fn docs() -> OpenApiDocs {
    let mut api = story::ApiDoc::openapi();
    api.merge(file::ApiDoc::openapi());
    api.merge(task::ApiDoc::openapi());
    api
}
