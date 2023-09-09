pub mod catalog_explorer;
pub mod catalogs;
pub mod fragments;
pub mod health_check;
pub mod video_details;
pub mod video_watch;

use std::{any::Any, sync::Arc};

use axum::{
    body::Body,
    extract::FromRef,
    http::{header, Request, Response, StatusCode},
    Router,
};
use axum_extra::routing::RouterExt;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    request_id::MakeRequestUuid,
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing::{info, instrument};

use crate::{configuration::Configuration, logging::Logger};

use self::catalog_explorer::catalog_explorer;
use self::catalogs::catalogs;
use self::fragments::test::test;
use self::health_check::health_check;
use self::video_details::video_details;
use self::video_watch::video_watch;

static REQUEST_ID_HEADER: &str = "x-request-id";
static MISSING_REQUEST_ID: &str = "missing_request_id";

#[derive(Clone, FromRef)]
pub struct AppState {
    config: Arc<Configuration>,
    database: PgPool,
}

impl AppState {
    pub fn new(config: Configuration, database: PgPool) -> Self {
        Self {
            config: Arc::new(config),
            database,
        }
    }
}

#[instrument(skip_all)]
pub fn init(config: Configuration, database: PgPool, _: &Logger) -> anyhow::Result<Router> {
    info!("initializing router...");
    let latency_unit = LatencyUnit::Micros;
    let http_tracing = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let path = request.uri().path();
            let method = request.method().as_ref();
            let request_id = request
                .headers()
                .get(REQUEST_ID_HEADER)
                .map_or(MISSING_REQUEST_ID, |value| {
                    value.to_str().unwrap_or(MISSING_REQUEST_ID)
                });
            tracing::info_span!("http", path, method, request_id)
        })
        .on_request(DefaultOnRequest::new())
        .on_response(DefaultOnResponse::new().latency_unit(latency_unit))
        .on_failure(DefaultOnFailure::new().latency_unit(latency_unit));

    let body_limit = RequestBodyLimitLayer::new(config.middlewares().body_size_limit());

    let allowed_origins = config.middlewares().allowed_origins()?;
    let cors = CorsLayer::permissive().allow_origin(allowed_origins);

    let timeout = TimeoutLayer::new(config.middlewares().request_timeout());

    let compression = CompressionLayer::new();

    let panic_handling = CatchPanicLayer::custom(handle_panic);

    let middlewares = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(http_tracing)
        .propagate_x_request_id()
        .layer(body_limit)
        .layer(cors)
        .layer(timeout)
        .layer(compression)
        .layer(panic_handling);
    #[cfg(debug_assertions)]
    let middlewares = middlewares
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            http::header::CACHE_CONTROL,
            http::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ))
        .layer(
            tower_livereload::LiveReloadLayer::new()
                .request_predicate::<String, live_reload_predicate::PathPredicate>(
                    live_reload_predicate::PathPredicate,
                ),
        );

    let static_dir = config.static_dir().to_owned();
    let state = AppState::new(config, database);

    let fragments = Router::new().typed_get(test);

    let router = Router::new()
        .typed_get(health_check)
        .typed_get(catalogs)
        .typed_get(catalog_explorer)
        .typed_get(video_details)
        .typed_get(video_watch)
        .nest("/fragments", fragments)
        .nest_service(
            "/assets",
            ServeDir::new(static_dir).call_fallback_on_method_not_allowed(true),
        )
        .layer(middlewares)
        .with_state(state);

    info!("initialized router");
    Ok(router)
}

#[instrument(skip_all)]
fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s
    } else {
        "unknown panic message"
    };

    tracing::error!("Service panicked: {details}");

    // TODO Change to error struct when ready
    let body = serde_json::json!({
        "error": {
            "kind": "panic",
            "message": details,
        }
    });
    let body = serde_json::to_string(&body)
        .expect("error in building response, handle_panic is probably misconfigured");

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .expect("error in building response, handle_panic is probably misconfigured")
}

#[cfg(debug_assertions)]
mod live_reload_predicate {
    use http::Request;
    use tower_livereload::predicate::Predicate;

    #[derive(Copy, Clone, Debug)]
    pub struct PathPredicate;

    impl<T> Predicate<Request<T>> for PathPredicate {
        fn check(&mut self, request: &Request<T>) -> bool {
            !(request.uri().to_string().starts_with("/fragments"))
        }
    }
}
