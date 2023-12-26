use std::time::Duration;

use anyhow::{bail, Context};
use elasticsearch::{
    http::{request::JsonBody, transport::Transport},
    BulkParts, Elasticsearch,
};
use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::PgPool;
use tap::Tap;
use tracing::{info, instrument};

use crate::{
    configuration::Configuration,
    crud::{Entity, StoreEntry},
    logging::Logger,
    model::{Catalog, Video},
};

#[instrument(skip_all)]
pub async fn init(config: &Configuration, _: &Logger) -> anyhow::Result<Elasticsearch> {
    info!("connecting to elastic...");
    let url = config.elastic().url();
    let transport = Transport::single_node(url.expose_secret())?;
    let client = Elasticsearch::new(transport);
    client
        .ping()
        .request_timeout(Duration::from_secs(10))
        .send()
        .await
        .context("could not ping elastic on provided url")?;
    Ok(client).tap(|_| info!("connected to elastic"))
}

#[instrument(skip_all)]
pub async fn index_database(
    elastic: &Elasticsearch,
    pool: &PgPool,
    _: &Logger,
) -> anyhow::Result<()> {
    info!("syncing elastic with database on startup...");

    index_entity::<Catalog>(elastic, pool, "catalogs")
        .await
        .context("could not index catalogs on startup")?;
    index_entity::<Video>(elastic, pool, "videos")
        .await
        .context("could not index videos on startup")?;

    Ok(()).tap(|_| info!("synced elastic with database"))
}

async fn index_entity<T: Entity + StoreEntry>(
    elastic: &Elasticsearch,
    pool: &PgPool,
    index_name: &str,
) -> anyhow::Result<()> {
    let mut to_index: Vec<JsonBody<_>> = vec![];

    T::find_all(pool, vec![], None)
        .await
        .with_context(|| format!("could not query {index_name}"))?
        .into_iter()
        .for_each(|entity| {
            to_index.push(json!({"index": {"_id": entity.id()}}).into());
            to_index.push(
                json!({
                    "path": entity.path(),
                    "display_name": entity.display_name(),
                    "short_desc": entity.short_desc(),
                    "long_desc": entity.long_desc()
                })
                .into(),
            );
        });

    if let Some(exception) = elastic
        .bulk(BulkParts::Index(index_name))
        .body(to_index)
        .send()
        .await
        .with_context(|| format!("could not sync {index_name} data into elastic, request failed"))?
        .exception()
        .await
        .with_context(|| format!("could not read {index_name} sync response body"))?
    {
        bail!(
            "could not sync {index_name} data - {}",
            exception.error().reason().unwrap_or("unknown reason")
        )
    };

    Ok(())
}
