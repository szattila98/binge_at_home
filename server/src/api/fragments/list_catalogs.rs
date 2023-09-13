use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, instrument};

use crate::{
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
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

#[derive(Error, Debug)]
pub enum ListCatalogsError {
    #[error("database error")]
    DbErr(#[from] sqlx::Error),
}

impl IntoResponse for ListCatalogsError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        let status_code = match self {
            ListCatalogsError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, msg).into_response()
    }
}

#[instrument(skip(pool))]
pub async fn list_catalogs(
    _: ListCatalogs,
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    sort: Option<Query<Sort<CatalogSort>>>,
) -> Result<impl IntoResponse, ListCatalogsError> {
    // TODO error handling into template - no need for pub - add to snippets
    let pagination = pagination.map(|Query(p)| p);
    let sort = sort.map(|Query(o)| vec![o]).unwrap_or_else(|| vec![]);
    let catalogs = Catalog::find_all(&pool, sort, pagination).await?;
    let rendered = ListCatalogsTemplate { catalogs };
    debug!("list catalogs rendered\n{rendered}");
    Ok(rendered)
}
