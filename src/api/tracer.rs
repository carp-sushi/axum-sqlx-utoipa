use crate::api::Ctx;
use axum::{extract::MatchedPath, http::Request, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Create a layered router with tower http tracing span.
pub(crate) fn wrap(routes: Router<Arc<Ctx>>) -> Router<Arc<Ctx>> {
    routes.layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let matched_path = req
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            tracing::info!("{} {:?}", req.method(), matched_path.unwrap_or_default());

            tracing::info_span!(
                "http_request",
                method = ?req.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        }),
    )
}
