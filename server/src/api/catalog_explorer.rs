use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(TypedPath)]
#[typed_path("/catalog/id/explorer")]
pub struct CatalogExplorerEndpoint;

#[derive(Serialize, Template)]
#[template(path = "catalog-explorer.html")]
struct CatalogExplorerTemplate;

#[instrument(skip_all)]
pub async fn catalog_explorer(_: CatalogExplorerEndpoint) -> impl IntoResponse {
    debug!("catalog explorer rendered");
    CatalogExplorerTemplate
}
