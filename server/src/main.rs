use binge_at_home::{
    api::init_router,
    configuration::Configuration,
    logging::{self},
    startup::Application,
};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::load()?;
    let logger = logging::init(&config)?;
    info!("loaded config and intialized logging");
    debug!("{config:#?}");
    let router = init_router(&logger)?;
    info!("initialized router");
    let app = Application::new(config, router, logger);
    app.run_until_stopped().await
}
