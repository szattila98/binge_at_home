use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::configuration::Configuration;

pub async fn init_database(config: &Configuration) -> anyhow::Result<PgPool> {
    let url = config.database().url();
    let pool = PgPoolOptions::new()
        .connect(url)
        .await
        .with_context(|| format!("could not establish connection to database on url '{url}'"))?;
    Ok(pool)
}
