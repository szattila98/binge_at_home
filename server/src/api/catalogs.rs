use std::{str::FromStr, sync::Arc};

use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::{extract::Query, routing::TypedPath};
use http::Uri;
use serde::Deserialize;
use sqlx::PgPool;
use tap::Tap;
use tracing::{debug, field, instrument};

use crate::{
    api::filters,
    configuration::Configuration,
    crud::{catalog::CatalogSort, Entity, Pagination, Sort},
    model::Catalog,
};

#[cfg(debug_assertions)]
use super::AppState;
use super::{
    include::{pager::PagerTemplate, sortable_header::SortableHeaderTemplate},
    technical_error::redirect_to_technical_error,
    PagedParams,
};

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default = "Params::default_page")]
    page: usize,
    sort: Vec<String>,
}

impl PagedParams for Params {
    fn get_page(&self) -> usize {
        self.page
    }
}

#[derive(Template)]
#[template(path = "catalogs.html")]
struct HtmlTemplate {
    catalogs: Vec<Catalog>,
    pager: PagerTemplate,
    sorts: Vec<Sort<CatalogSort>>,
    uri: Uri,
}

impl HtmlTemplate {
    pub fn get_sortable_header(
        &self,
        text: &'static str,
        catalog_sort: CatalogSort,
    ) -> SortableHeaderTemplate<CatalogSort> {
        let direction = match self.sorts.iter().find(|sort| sort.field == catalog_sort) {
            Some(sort) => Some(sort.clone()),
            None => None,
        }
        .map(|sort| sort.direction);
        SortableHeaderTemplate::new(
            text.to_string(),
            catalog_sort,
            direction,
            self.uri.to_string(),
        )
    }
}

impl HtmlTemplate {
    fn new(
        catalogs: Vec<Catalog>,
        pager: PagerTemplate,
        sorts: Vec<Sort<CatalogSort>>,
        uri: Uri,
    ) -> Self {
        Self {
            catalogs,
            pager,
            sorts,
            uri,
        }
        .tap(|template| debug!("rendered html template:\n{template}"))
    }
}

#[instrument(skip(config, pool, uri), fields(pager = field::Empty))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    params: Option<Query<Params>>,
    State(config): State<Arc<Configuration>>,
    State(pool): State<PgPool>,
    uri: Uri,
) -> Response {
    let Query(params) = params.unwrap_or_else(|| {
        Query(Params {
            page: 1,
            sort: vec![],
        })
    });
    let pagination = Some(Pagination {
        size: Params::page_size() as i64,
        page: params.page as i64,
    });
    let sorts = params
        .sort
        .iter()
        .filter_map(|s| Sort::<CatalogSort>::from_str(s).ok())
        .collect::<Vec<_>>();

    let (catalogs, number_of_catalogs) = tokio::join!(
        Catalog::find_all(&pool, sorts.clone(), pagination),
        Catalog::count_all(&pool)
    );
    let catalogs = match catalogs {
        Ok(catalogs) => catalogs,
        Err(error) => {
            return redirect_to_technical_error(&config, &error.to_string()).into_response();
        }
    };
    let number_of_catalogs = match number_of_catalogs {
        Ok(n) => n,
        Err(error) => {
            return redirect_to_technical_error(&config, &error.to_string()).into_response();
        }
    };

    let pager = params.create_pager(number_of_catalogs as usize, Endpoint::PATH.to_owned());

    HtmlTemplate::new(catalogs, pager, sorts, uri).into_response()
}
