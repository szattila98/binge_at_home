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
use tracing::{debug, error, field, instrument};

use crate::{
    api::technical_error::redirect_to_technical_error,
    configuration::Configuration,
    model::StoreEntry,
    search::{ElasticQueryResponse, MAX_QUERY_LEN},
};

use super::include::pager::PagerTemplate;

#[cfg(debug_assertions)]
use super::AppState;

#[derive(TypedPath)]
#[typed_path("/search")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    query: String,
    #[serde(default = "default_page")]
    page: usize,
}

const fn default_page() -> usize {
    1
}

#[derive(Serialize, Template)]
#[template(path = "search.html")]
struct HtmlTemplate {
    results: Vec<StoreEntry>,
    pager: PagerTemplate,
    query: String,
}

impl HtmlTemplate {
    fn new(results: Vec<StoreEntry>, pager: PagerTemplate, query: String) -> Self {
        Self {
            results,
            pager,
            query,
        }
        .tap(|template| debug!("rendered html template:\n{template}"))
    }
}

pub const SEARCH_PAGE_SIZE: usize = 10;

#[instrument(skip(config, elastic), fields(pager = field::Empty))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    params: Option<Query<Params>>,
    State(config): State<Arc<Configuration>>,
    State(elastic): State<Arc<Elasticsearch>>,
) -> Response {
    // FIXME strange optional
    let Query(Params { query, page }) = params.unwrap_or_else(|| {
        Query(Params {
            query: String::new(),
            page: 1,
        })
    });

    // TODO redirect if query is limited to max query length so url param is correct - maybe from request parts implementation
    // same with page, redirect to have it is different from starting one
    // TODO validate page is not too big
    let query = query[..MAX_QUERY_LEN.min(query.len())].to_owned();
    let page = (page - 1).max(0);

    let response = match elastic
        .search(SearchParts::Index(&["catalogs", "videos"]))
        .from((page * SEARCH_PAGE_SIZE) as i64)
        .size(SEARCH_PAGE_SIZE as i64)
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
                .into_response()
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
        return redirect_to_technical_error(&config, &msg)
            .tap(|_| error!("elastic query exception - reason: {msg}"))
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

    let pager = PagerTemplate::new(
        response.hits.total.value.div_ceil(SEARCH_PAGE_SIZE),
        page + 1,
        10,
        format!("{}?query={query}", Endpoint::PATH),
    );

    let results = response
        .hits
        .hits
        .into_iter()
        .map(|hit| hit.source)
        .collect();

    HtmlTemplate::new(results, pager, query).into_response()
}
