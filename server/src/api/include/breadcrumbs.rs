use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing_unwrap::OptionExt;

use crate::{api::browse::Endpoint as BrowseEndpoint, model::EntityId};

#[derive(Debug, Serialize)]
pub struct Breadcrumb {
    pub text: String,
    pub link: String,
}

impl Breadcrumb {
    pub fn new(catalog_id: EntityId, path: &[&str]) -> Self {
        let text = path
            .last()
            .expect_or_log("no last path part is found")
            .to_owned()
            .to_owned();
        // FIXME use with_query_params instead of replacing
        let link = BrowseEndpoint::PATH
            .replace(":catalog_id", &catalog_id.to_string())
            .replace("*path", &path.join("/"));
        Self { text, link }
    }
}

pub type Breadcrumbs = Vec<Breadcrumb>;

/// Creates breadcrumbs from a given file path. Breadcrumbs links point to browse endpoints.
pub fn extract_breadcrumbs(catalog_id: EntityId, path: &str) -> Breadcrumbs {
    let mut breadcrumbs = vec![];
    let mut path_parts = path.split('/').collect::<Vec<_>>();
    while !path_parts.is_empty() {
        breadcrumbs.push(Breadcrumb::new(catalog_id, &path_parts));
        let _ = path_parts.pop();
    }
    breadcrumbs.reverse();
    breadcrumbs
}
