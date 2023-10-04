use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{debug, instrument, warn};

use crate::{
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    model::{Catalog, FormatDate},
};

use super::AppState;

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct Endpoint;

#[derive(Serialize)]
enum TemplateState {
    Ok { catalogs: Vec<Catalog> },
    NoCatalogsFound,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "catalogs.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
) -> impl IntoResponse {
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map(|Query(o)| vec![o]).unwrap_or_else(Vec::new);

    let catalogs = match Catalog::find_all(&pool, sort, pagination).await {
        Ok(catalogs) => catalogs,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HtmlTemplate::new(TemplateState::DbErr(e.to_string())),
            )
        }
    };

    if catalogs.is_empty() {
        warn!("no catalogs found");
        return (
            StatusCode::NOT_FOUND,
            HtmlTemplate::new(TemplateState::NoCatalogsFound),
        );
    };

    let rendered = HtmlTemplate::new(TemplateState::Ok { catalogs });
    debug!("list catalogs rendered\n{rendered}");
    (StatusCode::OK, rendered)
}
