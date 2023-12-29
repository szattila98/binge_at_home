use std::{net::SocketAddr, sync::Arc};

use anyhow::Ok;
use binge_at_home::{
    api,
    configuration::Configuration,
    database::{self},
    file_access::{FileStore, StoreWatcher},
    logging::{self, with_default_logger},
    print_banner, search,
    startup::Application,
};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_banner();
    let (config, logger) = with_default_logger(|| {
        let config = Configuration::load()?;
        let logger = logging::init(&config)?;
        Ok((config, logger))
    })?;
    debug!("{config:#?}");
    let database = database::init(&config, &logger).await?;

    #[cfg(feature = "migrate")]
    sqlx::migrate!().run(&database).await?;

    let elastic = Arc::new(search::init(&config, &logger).await?);
    search::index_database(&elastic, &database, &logger).await?;

    let config = Arc::new(config);
    let file_store = Arc::new(FileStore::new(config.clone(), elastic.clone()));
    let mut store_watcher =
        StoreWatcher::new(config.clone(), file_store.clone(), database.clone()).await;
    store_watcher.watch_store()?;

    let address = SocketAddr::new(config.host(), config.port());
    let router = api::init(config, database, file_store, elastic, &logger)?;
    let app = Application::new(address, router, logger);
    app.run_until_stopped().await
}
