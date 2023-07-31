use std::net::SocketAddr;

use axum::{Router, Server};
use tokio::signal;

use anyhow::Context;
use tracing::info;

use crate::logging::Logger;

pub struct Application {
    address: SocketAddr,
    router: Router,
}

impl Application {
    pub fn new(address: SocketAddr, router: Router, _: Logger) -> Self {
        Self { address, router }
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        info!("Starting server on {}...", &self.address);
        Server::bind(&self.address)
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
