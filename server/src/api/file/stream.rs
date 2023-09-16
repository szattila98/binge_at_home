use std::sync::Arc;

use axum::{
    body::StreamBody,
    extract::State,
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::routing::TypedPath;
use http::{header, StatusCode};
use serde::Deserialize;
use sqlx::PgPool;
use tokio_util::io::ReaderStream;
use tracing::instrument;

use crate::{
    configuration::Configuration,
    crud::Entity,
    model::{EntityId, Video},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/video/:id/stream")]
pub struct Endpoint {
    id: EntityId,
}

#[instrument(skip(pool))]
pub async fn stream(
    Endpoint { id }: Endpoint,
    State(pool): State<PgPool>,
    State(config): State<Arc<Configuration>>,
) -> impl IntoResponse {
    // TODO legyenek matchek ahol resultot unwrapelek és átírt HtmlTemplate-re máshol is
    // TODO ne webm legyen hanem dinamikus itt és a browsenál is
    let result = Video::find(&pool, id).await;
    let Ok(opt) = result else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("database error: {}", result.unwrap_err().to_string()),
        ));
    };
    let Some(video) = opt else {
        return Err((
            StatusCode::NOT_FOUND,
            "video not found in database".to_string(),
        ));
    };

    let mut video_path = config.store();
    video_path.push(video.path());
    let file = match tokio::fs::File::open(video_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("video file not found: {err}"),
            ))
        }
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let headers = AppendHeaders([(header::CONTENT_TYPE, "text/webm")]);

    Ok((headers, body))
}
