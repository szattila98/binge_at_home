use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use http::StatusCode;
use macros::random_emoji;
use serde::Serialize;
use tracing::{debug, instrument};

use crate::{get_app_name, get_app_version};

use super::AppState;

#[derive(TypedPath)]
#[typed_path("/health-check")]
pub struct Endpoint;

#[derive(Serialize, Template)]
#[template(path = "health-check.html")]
struct HealthCheckTemplate {
    msg: &'static str,
    emoji: &'static str,
    app_name: &'static str,
    app_version: &'static str,
}

impl Default for HealthCheckTemplate {
    fn default() -> Self {
        Self {
            msg: "I am a happy and healthy service!",
            emoji: random_emoji!(),
            app_name: get_app_name(),
            app_version: get_app_version(),
        }
    }
}

#[instrument]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(_: Endpoint) -> impl IntoResponse {
    let rendered = HealthCheckTemplate::default();
    debug!("health check rendered\n{rendered}");
    (StatusCode::OK, rendered)
}
