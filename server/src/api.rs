use std::{any::Any, sync::Arc, time::Duration};

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

pub fn init_router(config: Configuration, database: PgPool, _: &Logger) -> anyhow::Result<Router> {
    let state = AppState::new(config, database);

    // TODO config
    let traffic_log_level = Level::INFO;

    let payload_size_limit = 4096;

    let allowed_headers = ["content-type".parse().unwrap()];
    let allowed_methods = ["GET".parse().unwrap(), "POST".parse().unwrap()];
    let allowed_origins = ["http://localhost:4000".parse().unwrap()];

    let request_timeout = Duration::from_secs(30);

    let middlewares = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let method = request.method().as_ref();
                    let request_id = request
                        .headers()
                        .get(REQUEST_ID_HEADER)
                        .map(|value| value.to_str().unwrap_or(MISSING_REQUEST_ID))
                        .unwrap_or(MISSING_REQUEST_ID);
                    tracing::info_span!("http", method, request_id)
                })
                .on_request(DefaultOnRequest::new().level(traffic_log_level))
                .on_response(
                    DefaultOnResponse::new()
                        .level(traffic_log_level)
                        .latency_unit(LatencyUnit::Micros),
                )
                .on_failure(
                    DefaultOnFailure::new()
                        .level(traffic_log_level)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .propagate_x_request_id()
        .layer(RequestBodyLimitLayer::new(payload_size_limit))
        .layer(
            CorsLayer::new()
                .allow_headers(allowed_headers)
                .allow_methods(allowed_methods)
                .allow_origin(allowed_origins),
        )
        .layer(TimeoutLayer::new(request_timeout))
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::custom(handle_panic));

    let router = Router::new()
        .route("/", get(health_check))
        .layer(middlewares)
        .with_state(state);

    // TODO add swagger and make it appear based on configuration

    Ok(router)
}

#[instrument]
async fn health_check() -> &'static str {
    info!("health check called, service is healthy");
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
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap()
}
