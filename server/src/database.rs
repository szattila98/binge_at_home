use std::time::Duration;

use anyhow::Context;
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{info, instrument};

use crate::{configuration::Configuration, logging::Logger};

#[instrument(skip_all)]
pub async fn init(config: &Configuration, _: &Logger) -> anyhow::Result<PgPool> {
    info!("connecting to database...");
    let url = config.database().url();
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(url.expose_secret())
        .await
        .context("could not establish connection to database on provided url")?;
    info!("connected to database");
    Ok(pool)
}
