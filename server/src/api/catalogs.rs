use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use serde::Serialize;
use sqlx::PgPool;
use tap::Tap;
use tracing::{debug, instrument, warn};

use crate::{
    configuration::Configuration,
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    model::{Catalog, FormatDate},
};

use super::technical_error::redirect_to_technical_error;
#[cfg(debug_assertions)]
use super::AppState;

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct Endpoint;

#[derive(Serialize)]
enum TemplateState {
    Ok { catalogs: Vec<Catalog> },
    NoCatalogsFound,
}

#[derive(Serialize, Template)]
#[template(path = "catalogs.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }.tap(|template| debug!("rendered html template:\n{template}"))
    }
}

#[instrument(skip(config, pool))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
    State(config): State<Arc<Configuration>>,
    State(pool): State<PgPool>,
) -> Response {
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map_or_else(Vec::new, |Query(o)| vec![o]);

    let catalogs = match Catalog::find_all(&pool, sort, pagination).await {
        Ok(catalogs) => catalogs,
        Err(error) => {
            return redirect_to_technical_error(&config, &error.to_string()).into_response();
        }
    };

    if catalogs.is_empty() {
        return HtmlTemplate::new(TemplateState::NoCatalogsFound)
            .tap(|_| warn!("no catalogs found"))
            .into_response();
    };

    HtmlTemplate::new(TemplateState::Ok { catalogs }).into_response()
}
