use std::{any::Any, sync::Arc};

use axum::{
    body::Body,
    extract::FromRef,
    http::{header, Request, Response, StatusCode},
    routing::get,
    Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing::{info, instrument, Level};

use crate::{configuration::Configuration, logging::Logger};

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

// TODO extract layer adding to functions
pub fn init_router(config: Configuration, database: PgPool, _: &Logger) -> anyhow::Result<Router> {
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
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(latency_unit),
        )
        .on_failure(
            DefaultOnFailure::new()
                .level(Level::INFO)
                .latency_unit(latency_unit),
        );

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

    let state = AppState::new(config, database);

    let router = Router::new()
        .route("/", get(health_check))
        .layer(middlewares)
        .with_state(state);

    // TODO add swagger and make it appear based on configuration

    Ok(router)
}

#[instrument]
async fn health_check() -> &'static str {
    info!("health check called");
    "I am healthy!"
}

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
