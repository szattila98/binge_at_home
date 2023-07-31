use anyhow::Context;
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{configuration::Configuration, logging::Logger};

pub async fn init(config: &Configuration, _: &Logger) -> anyhow::Result<PgPool> {
    let url = config.database().url();
    let pool = PgPoolOptions::new()
        .connect(url.expose_secret())
        .await
        .context("could not establish connection to database on provided url")?;
    Ok(pool)
}
