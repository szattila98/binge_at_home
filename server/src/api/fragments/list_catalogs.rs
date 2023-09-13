use askama::Template;
use axum::extract::{Query, State};
use axum_extra::routing::TypedPath;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{debug, error, instrument, warn};

use crate::{
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    model::Catalog,
};

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct Endpoint;

#[derive(Serialize)]
enum TemplateState {
    Success { catalogs: Vec<Catalog> },
    NoCatalogsFound,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "fragments/list-catalogs.html")]
pub struct PageTemplate {
    state: TemplateState,
}

impl PageTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
pub async fn list_catalogs(
    _: Endpoint,
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
) -> PageTemplate {
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map(|Query(o)| vec![o]).unwrap_or_else(|| vec![]);

    let result = Catalog::find_all(&pool, sort, pagination).await;
    let Ok(catalogs) = result else {
        let error = result.unwrap_err().to_string();
        error!("database error: {error}");
        return PageTemplate::new(TemplateState::DbErr(error));
    };

    if catalogs.is_empty() {
        warn!("no catalogs found");
        return PageTemplate::new(TemplateState::NoCatalogsFound);
    };

    let rendered = PageTemplate {
        state: TemplateState::Success { catalogs },
    };
    debug!("list catalogs rendered\n{rendered}");
    rendered
}
