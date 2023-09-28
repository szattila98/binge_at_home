use askama::Template;
use axum::extract::{Query, State};
use axum_extra::routing::TypedPath;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{debug, instrument, warn};

use crate::{
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    model::Catalog,
};

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
pub struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
pub async fn catalogs(
    _: Endpoint,
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
) -> HtmlTemplate {
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map(|Query(o)| vec![o]).unwrap_or_else(Vec::new);

    let result = Catalog::find_all(&pool, sort, pagination).await;
    let Ok(catalogs) = result else {
        return HtmlTemplate::new(TemplateState::DbErr(result.unwrap_err().to_string()));
    };

    if catalogs.is_empty() {
        warn!("no catalogs found");
        return HtmlTemplate::new(TemplateState::NoCatalogsFound);
    };

    let rendered = HtmlTemplate {
        state: TemplateState::Ok { catalogs },
    };
    debug!("list catalogs rendered\n{rendered}");
    rendered
}
