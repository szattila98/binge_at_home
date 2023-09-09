use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(TypedPath)]
#[typed_path("/video/id")]
pub struct VideoDetailsEndpoint;

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
struct VideoDetailsTemplate;

#[instrument(skip_all)]
pub async fn video_details(_: VideoDetailsEndpoint) -> impl IntoResponse {
    debug!("video details rendered");
    VideoDetailsTemplate
}
