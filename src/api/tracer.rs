use crate::api::Ctx;
use axum::{extract::MatchedPath, http::Request, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Create a layered router with tower http tracing span.
pub(crate) fn wrap(routes: Router<Arc<Ctx>>) -> Router<Arc<Ctx>> {
    routes.layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let path = req
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            tracing::info!("{} {:?}", req.method(), path.unwrap_or_default());

            tracing::info_span!(
                "request",
                method = ?req.method(),
                path,
                some_other_field = tracing::field::Empty,
            )
        }),
    )
}
