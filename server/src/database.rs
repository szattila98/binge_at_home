use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{configuration::Configuration, logging::Logger};

pub async fn init(config: &Configuration, _: &Logger) -> anyhow::Result<PgPool> {
    let url = config.database().url();
    let pool = PgPoolOptions::new()
        .connect(url)
        .await
        .with_context(|| format!("could not establish connection to database on url '{url}'"))?;
    Ok(pool)
}
