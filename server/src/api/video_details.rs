use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, instrument};

use crate::{
    crud::Entity,
    model::{EntityId, Video},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/video/:id")]
pub struct VideoDetailsEndpoint {
    id: EntityId,
}

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
struct VideoDetailsTemplate {
    video: Video,
}

#[derive(Error, Debug)]
pub enum VideoDetailsError {
    #[error("video not found")]
    VideoNotFound,
    #[error("database error")]
    DbErr(#[from] sqlx::Error),
}

impl IntoResponse for VideoDetailsError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        let status_code = match self {
            VideoDetailsError::VideoNotFound => StatusCode::NOT_FOUND,
            VideoDetailsError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, msg).into_response()
    }
}

#[instrument]
pub async fn video_details(
    VideoDetailsEndpoint { id }: VideoDetailsEndpoint,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, VideoDetailsError> {
    let Some(video) = Video::find(&pool, id).await? else {
        return Err(VideoDetailsError::VideoNotFound);
    };
    let rendered = VideoDetailsTemplate { video };
    debug!("video details rendered\n{rendered}");
    Ok(rendered)
}
