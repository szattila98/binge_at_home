use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use macros::random_emoji;
use serde::Serialize;
use tracing::{info, instrument};

use crate::{get_app_name, get_app_version};

#[derive(TypedPath)]
#[typed_path("/health-check")]
pub struct HealthCheckEndpoint;

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

#[instrument(skip_all)]
pub async fn health_check(_: HealthCheckEndpoint) -> impl IntoResponse {
    info!("health check called");
    HealthCheckTemplate::default()
}
