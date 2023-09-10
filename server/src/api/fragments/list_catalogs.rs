use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::routing::TypedPath;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{debug, error, instrument};

use crate::{
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    error::AppError,
    model::Catalog,
};

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct ListCatalogs;

#[derive(Serialize, Template)]
#[template(path = "fragments/list-catalogs.html")]
struct ListCatalogsTemplate {
    catalogs: Vec<Catalog>,
}

#[instrument(skip_all)]
pub async fn list_catalogs(
    _: ListCatalogs,
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
) -> Result<impl IntoResponse, AppError<sqlx::Error>> {
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map(|Query(o)| vec![o]).unwrap_or_else(|| vec![]);
    let catalogs = Catalog::find_all(&pool, sort, pagination)
        .await
        .map_err(|e| {
            error!("error while listing catalogs: {e}");
            e
        })?;
    let rendered = ListCatalogsTemplate { catalogs };
    debug!("list catalogs rendered\n{rendered}");
    Ok(rendered)
}
