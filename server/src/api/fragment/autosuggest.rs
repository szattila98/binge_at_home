use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::routing::TypedPath;
use elasticsearch::{Elasticsearch, SearchParts};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error, instrument};

use crate::{
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

#[derive(Serialize)]
enum TemplateState {
    Ok { results: Vec<StoreEntry> },
    TechnicalError(String),
}

#[derive(Serialize, Template)]
#[template(path = "fragments/autosuggest.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(elastic))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    Query(Params { query }): Query<Params>,
    State(elastic): State<Arc<Elasticsearch>>,
) -> impl IntoResponse {
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
            error!("elastic query error - {error}");
            return HtmlTemplate::new(TemplateState::TechnicalError(error.to_string()));
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
        error!("elastic query exception - reason: {msg}");
        return HtmlTemplate::new(TemplateState::TechnicalError(msg.to_owned()));
    };

    let response = match response.json::<ElasticQueryResponse<StoreEntry>>().await {
        Ok(response) => response,
        Err(error) => {
            error!("elastic response deserialization failed - {error}");
            return HtmlTemplate::new(TemplateState::TechnicalError(
                "could not deserialize elastic response".to_owned(),
            ));
        }
    };
    debug!("elastic query took {}ms to complete", response.took);

    let results = response
        .hits
        .hits
        .into_iter()
        .map(|hit| hit.source)
        .collect();

    HtmlTemplate::new(TemplateState::Ok { results })
}
