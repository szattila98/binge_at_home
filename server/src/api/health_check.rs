use axum::{response::IntoResponse, Json};
use serde::Serialize;
use tracing::{info, instrument};

use crate::{get_app_name, get_app_version};

#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "Checks health of the service")
    )
)]
#[instrument]
pub async fn health_check() -> impl IntoResponse {
    info!("health check called");
    Json(HealthCheckResponse::default())
}

#[derive(Serialize)]
struct HealthCheckResponse {
    msg: &'static str,
    app_name: &'static str,
    app_version: &'static str,
}

impl Default for HealthCheckResponse {
    fn default() -> Self {
        Self {
            msg: "I am a happy and healthy service! 🦦",
            app_name: get_app_name(),
            app_version: get_app_version(),
        }
    }
}
