use askama::Template;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tap::Tap;
use tracing::debug;
use tracing_unwrap::OptionExt;

use crate::{
    api::browse::Endpoint as BrowseEndpoint, api::catalogs::Endpoint as CatalogsEndpoint,
    model::EntityId,
};

#[derive(Debug, Serialize)]
pub struct Breadcrumb {
    pub text: String,
    pub link: String,
}

#[derive(Serialize, Template)]
#[template(path = "includes/breadcrumbs.html")]
pub struct BreadcrumbsTemplate {
    breadcrumbs: Vec<Breadcrumb>,
}

impl BreadcrumbsTemplate {
    pub fn new(catalog_id: EntityId, path: &str) -> Self {
        let mut breadcrumbs = vec![Breadcrumb {
            text: "Home".to_owned(),
            link: CatalogsEndpoint::PATH.to_owned(),
        }];

        let mut path_parts = path.split('/').collect::<Vec<_>>();
        while !path_parts.is_empty() {
            breadcrumbs.insert(1, {
                let path = path_parts.join("/");
                let text = path_parts
                    .pop()
                    .expect_or_log("no last path part is found for breadcrumb")
                    .to_owned();
                let link = BrowseEndpoint::PATH
                    .replace(":catalog_id", &catalog_id.to_string())
                    .replace("*path", &path);
                Breadcrumb { text, link }
            });
        }

        BreadcrumbsTemplate { breadcrumbs }
            .tap(|template| debug!("rendered html include template:\n{template}"))
    }
}
