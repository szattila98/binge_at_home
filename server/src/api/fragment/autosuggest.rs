use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use elasticsearch::{Elasticsearch, SearchParts};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tap::Tap;
use tracing::{debug, error, instrument};

use crate::{
    api::technical_error::redirect_to_technical_error,
    configuration::Configuration,
    model::StoreEntry,
    search::{ElasticQueryResponse, MAX_QUERY_LEN},
};

#[cfg(debug_assertions)]
use super::super::AppState;

#[derive(TypedPath)]
#[typed_path("/search/autosuggest")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    query: String,
}

#[derive(Serialize, Template)]
#[template(path = "fragments/autosuggest.html")]
struct HtmlTemplate {
    results: Vec<StoreEntry>,
}

impl HtmlTemplate {
    fn new(results: Vec<StoreEntry>) -> Self {
        Self { results }.tap(|fragment| debug!("rendered html fragment:\n{fragment}"))
    }
}

#[instrument(skip(elastic))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    Query(Params { query }): Query<Params>,
    State(config): State<Arc<Configuration>>,
    State(elastic): State<Arc<Elasticsearch>>,
) -> Response {
    let query = query[..MAX_QUERY_LEN.min(query.len())].to_owned();

    let response = match elastic
        .search(SearchParts::Index(&["catalogs", "videos"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "query_string": {
                    "query": format!("*{}*", query),
                    "fields": ["path^0.5", "display_name^2", "short_desc^1", "long_desc^1"],
                    "fuzziness": 2
                }
            }
        }))
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return redirect_to_technical_error(&config, &error)
                .tap(|_| error!("elastic query error - {error}"))
                .into_response();
        }
    };

    if response.status_code() >= StatusCode::BAD_REQUEST {
        const UNKNOWN_REASON: &str = "unknown elastic exception reason";
        let msg = if let Ok(Some(exception)) = response.exception().await {
            exception
                .error()
                .reason()
                .unwrap_or(UNKNOWN_REASON)
                .to_owned()
        } else {
            UNKNOWN_REASON.to_owned()
        };
        let reason = msg.to_owned();
        return redirect_to_technical_error(&config, &reason)
            .tap(|_| error!("elastic query exception - reason: {reason}"))
            .into_response();
    };

    let response = match response.json::<ElasticQueryResponse<StoreEntry>>().await {
        Ok(response) => response,
        Err(error) => {
            return redirect_to_technical_error(&config, "could not deserialize elastic response")
                .tap(|_| error!("elastic response deserialization failed - {error}"))
                .into_response();
        }
    };
    debug!("elastic query took {}ms to complete", response.took);

    let results = response
        .hits
        .hits
        .into_iter()
        .map(|hit| hit.source)
        .collect();

    HtmlTemplate::new(results).into_response()
}
