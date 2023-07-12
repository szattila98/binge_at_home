use axum::{routing::get, Router};
use tracing::debug;

use crate::logging::Logger;

pub fn init_router(_: &Logger) -> anyhow::Result<Router> {
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
