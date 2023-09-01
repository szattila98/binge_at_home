use axum::{response::IntoResponse, Json};
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{info, instrument};

use crate::{get_app_name, get_app_version};

#[derive(TypedPath)]
#[typed_path("/")]
pub struct HealthCheckEndpoint;

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

#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "Checks health of the service")
    )
)]
#[instrument(skip_all)]
pub async fn health_check(_: HealthCheckEndpoint) -> impl IntoResponse {
    info!("health check called");
    Json(HealthCheckResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn health_check_is_ok() {
        let response = health_check(HealthCheckEndpoint).await;
        assert_eq!(response.into_response().status(), StatusCode::OK)
    }
}
