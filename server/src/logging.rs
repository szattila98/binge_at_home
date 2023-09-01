use std::{marker::PhantomData, str::FromStr};

use anyhow::{bail, Context as AnyhowContext};
use tokio::task::JoinHandle;
use tracing::{
    info, instrument, subscriber::with_default, subscriber::Interest, Level, Subscriber,
};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, registry::LookupSpan, util::SubscriberInitExt,
    Layer,
};

use crate::configuration::Configuration;

pub fn with_default_logger<T>(f: impl Fn() -> T) -> T {
    let default_logger = tracing_subscriber::fmt().pretty().finish();
    with_default(default_logger, f)
}

pub struct Logger(PhantomData<Logger>);

#[instrument(skip_all)]
pub fn init(config: &Configuration) -> anyhow::Result<Logger> {
    let log_level =
        Level::from_str(config.logging().level()).context("log level could not be parsed")?;
    let global_filter = GlobalFilterLayer::new(log_level).boxed();
    info!("initializing logging with level '{log_level}'...");

    let mut layers = vec![];

    let stdout_logger = tracing_subscriber::fmt::layer().pretty().boxed();
    layers.push(stdout_logger);

    let file_logger = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(false)
        .with_writer(tracing_appender::rolling::daily(
            config.logging().file().dir(),
            config.logging().file().name(),
        ))
        .boxed();
    layers.push(file_logger);

    if let Err(e) = tracing_subscriber::registry()
        .with(global_filter)
        .with(layers)
        .try_init()
    {
        bail!("logger could not be initialized: {e}")
    };
    info!("initialized logging");
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

struct GlobalFilterLayer {
    log_level: Level,
}

impl GlobalFilterLayer {
    fn new(log_level: Level) -> Self {
        Self { log_level }
    }
}

impl<S> Layer<S> for GlobalFilterLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        let is_configured_log_level = *metadata.level() <= self.log_level;
        let is_hyper_debug_log = metadata
            .module_path()
            .map_or(false, |path| path.starts_with("hyper"));
        let enabled = is_configured_log_level && !is_hyper_debug_log;
        if enabled {
            Interest::always()
        } else {
            Interest::never()
        }
    }
}
