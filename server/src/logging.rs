use std::{marker::PhantomData, str::FromStr};

use anyhow::{bail, Context};
use tokio::task::JoinHandle;
use tracing::Level;
use tracing_subscriber::{
    filter::filter_fn, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::configuration::Configuration;

pub struct Logger(PhantomData<Logger>);

pub fn init(config: &Configuration) -> anyhow::Result<Logger> {
    let log_level =
        Level::from_str(config.logging().level()).context("log level could not be parsed")?;

    let registry = tracing_subscriber::registry();

    let default_filter = filter_fn(move |metadata| {
        let is_configured_log_level = *metadata.level() <= log_level;
        let is_hyper_debug_log = metadata
            .module_path()
            .map_or(false, |path| path.starts_with("hyper"));
        is_configured_log_level && !is_hyper_debug_log
    });

    let stdout_logger = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(default_filter.clone());

    let file_logger = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(false)
        .with_writer(tracing_appender::rolling::daily(
            config.logging().file().dir(),
            config.logging().file().name(),
        ))
        .with_filter(default_filter);

    let debug_file_logger = if config.logging().file().separate_debug_file() {
        Some(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(tracing_appender::rolling::never(
                    config.logging().file().dir(),
                    "debug.log",
                ))
                .with_filter(filter_fn(move |metadata| {
                    let is_under_debug = *metadata.level() <= Level::DEBUG;
                    let is_hyper_debug_log = metadata
                        .module_path()
                        .map_or(false, |path| path.starts_with("hyper"));
                    is_under_debug && !is_hyper_debug_log
                })),
        )
    } else {
        None
    };

    if let Err(e) = registry
        .with(stdout_logger)
        .with(file_logger)
        .with(debug_file_logger)
        .try_init()
    {
        bail!("logger could not be initialized: {e}")
    };
    Ok(Logger(PhantomData))
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
