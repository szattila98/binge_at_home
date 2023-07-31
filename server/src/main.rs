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

use std::net::SocketAddr;

use binge_at_home::{
    api::init_router,
    configuration::Configuration,
    database::{self},
    logging::{self},
    print_banner,
    startup::Application,
};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_banner();
    let config = Configuration::load()?;
    let logger = logging::init(&config)?;
    let address = SocketAddr::new(config.host(), config.port());
    info!("loaded config and initialized logging");
    debug!("{config:#?}");
    info!("connecting to database...");
    let database = database::init(&config, &logger).await?;
    info!("connected to database");
    info!("initializing router...");
    let router = init_router(config, database, &logger)?;
    info!("initialized router");
    let app = Application::new(address, router, logger);
    app.run_until_stopped().await
}
