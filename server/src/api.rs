pub mod browse;
pub mod catalogs;
pub mod file;
pub mod fragment;
pub mod health_check;
pub mod video_details;
pub mod video_watch;

use std::{any::Any, sync::Arc};

use axum::{
    body::Body,
    extract::FromRef,
    http::{header, Request, Response, StatusCode},
    response::Redirect,
    routing::get,
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

use crate::{
    api::file::{scan_store, stream},
    configuration::Configuration,
    file_access::FileStore,
    logging::Logger,
};

static REQUEST_ID_HEADER: &str = "x-request-id";
static MISSING_REQUEST_ID: &str = "missing_request_id";

#[derive(Clone, FromRef)]
pub struct AppState {
    config: Arc<Configuration>,
    database: PgPool,
    file_store: Arc<FileStore>,
}

impl AppState {
    pub fn new(config: Configuration, database: PgPool, file_store: Arc<FileStore>) -> Self {
        Self {
            config: Arc::new(config),
            database,
            file_store,
        }
    }
}

#[instrument(skip_all)]
pub fn init(
    config: Configuration,
    database: PgPool,
    file_store: Arc<FileStore>,
    _: &Logger,
) -> anyhow::Result<Router> {
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
                .request_predicate::<String, live_reload_predicate::RequestPredicate>(
                    live_reload_predicate::RequestPredicate,
                ),
        );

    let static_dir = config.static_dir().to_owned();
    let state = AppState::new(config, database, file_store);

    let fragment = Router::new();

    let file = Router::new()
        .typed_get(stream::handler)
        .typed_get(scan_store::handler);

    let router = Router::new()
        .route("/", get(|| async { Redirect::permanent("/catalog") }))
        .typed_get(health_check::handler)
        .typed_get(catalogs::handler)
        .typed_get(browse::handler)
        .typed_get(video_details::handler)
        .typed_get(video_watch::handler)
        .nest("/fragment", fragment)
        .nest("/file", file)
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

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json") // TODO use askama server error template instead
        .body(Body::from(format!("fatal error: {details}")))
        .expect("error in building response, handle_panic is probably misconfigured")
}

#[cfg(debug_assertions)]
mod live_reload_predicate {
    use http::Request;
    use tower_livereload::predicate::Predicate;

    #[derive(Copy, Clone, Debug)]
    pub struct RequestPredicate;

    impl<T> Predicate<Request<T>> for RequestPredicate {
        fn check(&mut self, request: &Request<T>) -> bool {
            request.headers().get("Hx-Request").is_none()
        }
    }
}
