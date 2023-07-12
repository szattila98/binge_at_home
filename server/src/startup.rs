use std::net::SocketAddr;

use axum::{Router, Server};
use tokio::signal;

use anyhow::Context;
use tracing::info;

use crate::{configuration::Configuration, logging::Logger};

pub struct Application {
    configuration: Configuration,
    router: Router,
}

impl Application {
    pub fn new(configuration: Configuration, router: Router, _: Logger) -> Self {
        Self {
            configuration,
            router,
        }
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        let server_address = SocketAddr::new(self.configuration.host(), self.configuration.port());
        info!("Starting server on {}...", server_address);
        Server::bind(&server_address)
            .serve(self.router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("server failed to initialize")?;
        info!("Shutting down server...");
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
