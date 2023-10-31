#![deny(unsafe_code)]
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::mod_module_files,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::suspicious_operation_groupings,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

use std::{net::SocketAddr, sync::Arc};

use anyhow::Ok;
use binge_at_home::{
    api::init,
    configuration::Configuration,
    database::{self},
    file_access::{FileStore, StoreWatcher},
    logging::{self, with_default_logger},
    print_banner,
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

    let file_store = Arc::new(FileStore::new(Arc::new(config.clone())));
    let mut store_watcher =
        StoreWatcher::new(config.clone(), file_store.clone(), database.clone()).await;
    store_watcher.watch_store()?;

    let address = SocketAddr::new(config.host(), config.port());
    let router = init(config, database, file_store, &logger)?;
    let app = Application::new(address, router, logger);
    app.run_until_stopped().await
}
