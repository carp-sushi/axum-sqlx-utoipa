use crate::api::Ctx;
use axum::{
    extract::{DefaultBodyLimit, MatchedPath},
    http::{Request, Response},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

// Cap file upload size
const BODY_LIMIT: usize = 250 * 1000 * 1000; // ~250m

/// Create a layered router with tower http request tracing.
pub(crate) fn wrap(routes: Router<Arc<Ctx>>) -> Router<Arc<Ctx>> {
    routes
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(BODY_LIMIT))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request<_>| {
                    let path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    tracing::info_span!("request", method = ?req.method(), path)
                })
                .on_response(|res: &Response<_>, lat: Duration, _: &Span| {
                    tracing::info!("response status = {}, latency = {:?}", res.status(), lat);
                }),
        )
}
