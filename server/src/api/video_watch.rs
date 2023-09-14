use askama::Template;
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use tracing::{debug, instrument};

#[derive(TypedPath)]
#[typed_path("/video/id/watch")]
pub struct VideoWatchEndpoint;

#[derive(Serialize, Template)]
#[template(path = "video-watch.html")]
struct VideoWatchTemplate;

#[instrument]
pub async fn video_watch(_: VideoWatchEndpoint) -> impl IntoResponse {
    let rendered = VideoWatchTemplate;
    debug!("video watch rendered\n{rendered}");
    rendered
}
