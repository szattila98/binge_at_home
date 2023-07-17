use std::sync::Arc;

use axum::{extract::FromRef, routing::get, Router};
use sqlx::PgPool;
use tracing::debug;

use crate::{configuration::Configuration, logging::Logger};

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

    let router = Router::new()
        .route(
            "/",
            get(|| async {
                debug!("health check called");
                "I am healthy!"
            }),
        )
        .with_state(state);

    // TODO add swagger and make it appear based on configuration

    Ok(router)
}
