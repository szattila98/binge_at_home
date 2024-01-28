use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tap::Tap;
use tracing::{debug, field, instrument};

use crate::{
    api::filters,
    configuration::Configuration,
    crud::{Entity, Pagination},
    model::Catalog,
};

#[cfg(debug_assertions)]
use super::AppState;
use super::{
    include::pager::PagerTemplate, technical_error::redirect_to_technical_error, PagedParams,
};

#[derive(TypedPath)]
#[typed_path("/catalog")]
pub struct Endpoint;

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default = "Params::default_page")]
    page: usize,
}

impl PagedParams for Params {
    fn get_page(&self) -> usize {
        self.page
    }
}

#[derive(Serialize, Template)]
#[template(path = "catalogs.html")]
struct HtmlTemplate {
    catalogs: Vec<Catalog>,
    pager: PagerTemplate,
}

impl HtmlTemplate {
    fn new(catalogs: Vec<Catalog>, pager: PagerTemplate) -> Self {
        Self { catalogs, pager }.tap(|template| debug!("rendered html template:\n{template}"))
    }
}

#[instrument(skip(config, pool), fields(pager = field::Empty))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    params: Option<Query<Params>>,
    State(config): State<Arc<Configuration>>,
    State(pool): State<PgPool>,
) -> Response {
    let Query(params) = params.unwrap_or_else(|| Query(Params { page: 1 }));
    let pagination = Some(Pagination {
        size: Params::page_size() as i64,
        page: params.page as i64,
    });

    let (catalogs, number_of_catalogs) = tokio::join!(
        Catalog::find_all(&pool, vec![], pagination),
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

    HtmlTemplate::new(catalogs, pager).into_response()
}
