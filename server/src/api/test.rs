use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{info, instrument};

#[derive(TypedPath)]
#[typed_path("/test")]
pub struct TestEndpoint;

#[derive(Serialize, Template)]
#[template(path = "fragments/test.html")]
struct TestTemplate;

#[instrument(skip_all)]
pub async fn test(_: TestEndpoint) -> impl IntoResponse {
    info!("test called");
    TestTemplate
}
