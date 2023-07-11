use binge_at_home::{
    api::init_router, configuration::Configuration, logging, startup::Application,
};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO make it impossible to mess up initialization by compile time checks, remove bails
    // TODO initialize a trace logging config before replacing it with configured logging so until then logs can be used https://docs.rs/tracing-subscriber/latest/tracing_subscriber/reload/index.html
    let config = Configuration::load()?;
    logging::init(&config)?;
    info!("loaded config and intialized logging");
    debug!("{config:#?}");
    let router = init_router()?;
    info!("initialized router");
    let app = Application::new(config, router);
    app.run_until_stopped().await
}
