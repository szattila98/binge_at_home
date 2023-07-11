use anyhow::bail;
use axum::{routing::get, Router};
use tracing::debug;

use crate::logging::is_logging_initialized;

pub fn init_router() -> anyhow::Result<Router> {
    if !is_logging_initialized() {
        bail!("logging should be initialized to create a router");
    }

    let router = Router::new().route(
        "/",
        get(|| async {
            debug!("health check called");
            "I am healthy!"
        }),
    );

    // TODO add swagger and make it appear based on configuration

    Ok(router)
}
