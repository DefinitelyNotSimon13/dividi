#[macro_export]
macro_rules! request_tracing {
    () => {
        tower_http::trace::TraceLayer::new_for_http()/*.make_span_with(|request: &axum::http::Request<_>| {
            let matched_path = request
                .extensions()
                .get::<axum::extract::MatchedPath>()
                .map(axum::extract::MatchedPath::as_str);

            tracing::trace_span!(
                "http_request",
                    method = ?request.method(),
                    matched_path,
            )
        })*/

    };
}
