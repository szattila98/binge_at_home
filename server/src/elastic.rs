use std::time::Duration;

use anyhow::{bail, Context};
use elasticsearch::{
    http::{request::JsonBody, transport::Transport},
    BulkParts, Elasticsearch,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value};
use sqlx::PgPool;
use tap::Tap;
use tracing::{info, instrument};
use tracing_unwrap::ResultExt;

use crate::{
    configuration::Configuration,
    crud::Entity,
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

    let (catalogs, videos) = tokio::join!(
        Catalog::find_all(pool, vec![], None),
        Video::find_all(pool, vec![], None)
    );
    index(elastic, catalogs?)
        .await
        .context("could not index catalogs on startup")?;
    index(elastic, videos?)
        .await
        .context("could not index videos on startup")?;

    Ok(()).tap(|_| info!("synced elastic with database"))
}

pub trait Indexable {
    fn index_name() -> &'static str;
}

pub async fn index<T>(
    elastic: &Elasticsearch,
    entities: impl IntoIterator<Item = T>,
) -> anyhow::Result<()>
where
    T: Entity + Serialize + Indexable,
{
    let to_index: Vec<JsonBody<_>> = entities
        .into_iter()
        .flat_map(|entity| {
            [
                json!({"index": {"_id": entity.id()}}).into(),
                to_value(entity)
                    .expect_or_log("could not serialize entity")
                    .into(),
            ]
        })
        .collect();

    let index_name = T::index_name();
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ElasticQueryResponse<T> {
    pub took: u64,
    pub hits: Hits<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits<T> {
    pub total: Total,
    pub max_score: Option<f64>,
    pub hits: Vec<Hit<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Total {
    pub value: u64,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hit<T> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: Option<f64>,
    #[serde(rename = "_source")]
    pub source: T,
}
