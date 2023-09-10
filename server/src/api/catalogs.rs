use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(TypedPath)]
#[typed_path("/")]
pub struct CatalogsEndpoint;

#[derive(Serialize, Template)]
#[template(path = "catalogs.html")]
struct CatalogsTemplate;

#[instrument(skip_all)]
pub async fn catalogs(_: CatalogsEndpoint) -> impl IntoResponse {
    let rendered = CatalogsTemplate;
    debug!("catalogs rendered\n{rendered}");
    rendered
}
