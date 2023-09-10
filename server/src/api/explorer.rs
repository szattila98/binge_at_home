use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(TypedPath)]
#[typed_path("/")]
pub struct ExplorerEndpoint;

#[derive(Serialize, Template)]
#[template(path = "explorer.html")]
struct ExplorerTemplate;

#[instrument]
pub async fn explorer(_: ExplorerEndpoint) -> impl IntoResponse {
    let rendered = ExplorerTemplate;
    debug!("catalogs rendered\n{rendered}");
    rendered
}
