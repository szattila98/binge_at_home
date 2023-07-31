use tracing::{info, instrument};

#[instrument]
#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "Checks health of the service")
    )
)]
pub async fn health_check() -> &'static str {
    info!("health check called");
    "I am healthy!"
}
