use axum::{extract::DefaultBodyLimit, Router};
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::{openapi::OpenApi as OpenApiDocs, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod ctx;
pub use ctx::Ctx;
mod dto;
mod routes;
use routes::{file, status, story, task};
mod tracer;

// Cap file upload size
const BODY_LIMIT: usize = 250 * 1000 * 1000;

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
                .layer(DefaultBodyLimit::disable())
                .layer(RequestBodyLimitLayer::new(BODY_LIMIT))
                .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs()))
                .merge(status::routes())
                .merge(story::routes())
                .merge(file::routes())
                .merge(task::routes()),
        )
        .with_state(self.ctx)
    }
}

/// Combine OpenApi docs for internal routes.
pub fn docs() -> OpenApiDocs {
    let mut api = story::ApiDoc::openapi();
    api.merge(file::ApiDoc::openapi());
    api.merge(task::ApiDoc::openapi());
    api
}
