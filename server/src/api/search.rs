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
use tap::Tap;
use tracing::{debug, error, instrument};

use crate::{
    elastic::ElasticQueryResponse,
    model::{Catalog, Video},
};

use super::AppState;

#[derive(TypedPath)]
#[typed_path("/search")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    query: String,
    #[serde(default = "default_page")]
    page: i64,
}

const fn default_page() -> i64 {
    1
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoreEntry {
    Video(Video),
    Catalog(Catalog),
}

#[derive(Serialize)]
enum TemplateState {
    Ok { results: Vec<StoreEntry> },
    Empty,
    TechnicalError(String),
}

#[derive(Serialize, Template)]
#[template(path = "search.html")]
struct HtmlTemplate {
    state: TemplateState,
    query: String,
}

impl HtmlTemplate {
    fn new(state: TemplateState, query: String) -> Self {
        Self { state, query }.tap(|rendered| debug!("search rendered\n{rendered}"))
    }
}

pub const SEARCH_PAGE_SIZE: i64 = 10;
pub const MAX_QUERY_LEN: usize = 30;

#[instrument(skip(elastic))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    params: Option<Query<Params>>,
    State(elastic): State<Arc<Elasticsearch>>,
) -> impl IntoResponse {
    let Some(Query(Params { query, page })) = params else {
        return HtmlTemplate::new(TemplateState::Empty, String::new());
    };

    // TODO redirect if query is limited to max query length so url param is correct - maybe from request parts implementation
    // same with page, redirect to have it is different from starting one
    let query = query[..MAX_QUERY_LEN.min(query.len())].to_owned();
    let page = (page - 1).max(0);

    let response = match elastic
        .search(SearchParts::Index(&["catalogs", "videos"]))
        .from(page * SEARCH_PAGE_SIZE)
        .size(SEARCH_PAGE_SIZE)
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
            return HtmlTemplate::new(
                TemplateState::TechnicalError(error.to_string()),
                String::new(),
            );
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
        return HtmlTemplate::new(TemplateState::TechnicalError(msg.to_owned()), String::new());
    };

    let response = match response.json::<ElasticQueryResponse<StoreEntry>>().await {
        Ok(response) => response,
        Err(error) => {
            error!("elastic response deserialization failed - {error}");
            return HtmlTemplate::new(
                TemplateState::TechnicalError("could not deserialize elastic response".to_owned()),
                String::new(),
            );
        }
    };
    debug!("elastic query took {}ms to complete", response.took);

    // FIXME pagination rendering and functionality
    let results = response
        .hits
        .hits
        .into_iter()
        .map(|hit| hit.source)
        .collect();

    HtmlTemplate::new(TemplateState::Ok { results }, query)
}
