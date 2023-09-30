use std::{io::SeekFrom, ops::Bound, sync::Arc};

use axum::{
    extract::State,
    headers::Range,
    response::{IntoResponse, Response},
    TypedHeader,
};
use axum_extra::routing::TypedPath;
use http::{header, StatusCode};
use http_body::Full;
use serde::Deserialize;
use sqlx::PgPool;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::instrument;

use crate::{
    configuration::Configuration,
    crud::Entity,
    model::{EntityId, Video},
};

static MAX_CHUNK_SIZE: u64 = 1024 * 1024;

#[derive(TypedPath, Deserialize)]
#[typed_path("/video/:id/stream")]
pub struct Endpoint {
    id: EntityId,
}

#[instrument(skip(pool))]
pub async fn handler(
    Endpoint { id }: Endpoint,
    TypedHeader(range_header): TypedHeader<Range>,
    State(pool): State<PgPool>,
    State(config): State<Arc<Configuration>>,
) -> impl IntoResponse {
    // TODO better error handling, unwraps
    let option = match Video::find(&pool, id).await {
        Ok(option) => option,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {e}"),
            ));
        }
    };
    let Some(video) = option else {
        return Err((
            StatusCode::NOT_FOUND,
            "video not found in database".to_string(),
        ));
    };

    let mut video_path = config.store(); // TODO video store struct may be needed
    video_path.push(video.path());
    let mut file = match tokio::fs::File::open(video_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("video file not found: {err}"),
            ))
        }
    };

    let file_size = file.metadata().await.unwrap().len(); // TODO use stored length

    let (range_start, range_end) = range_header.iter().next().unwrap();
    let range_start = match range_start {
        Bound::Included(range_start) | Bound::Excluded(range_start) => range_start,
        Bound::Unbounded => 0,
    };
    let range_end = match range_end {
        Bound::Included(range_end) | Bound::Excluded(range_end) => range_end,
        Bound::Unbounded => u64::min(range_start + MAX_CHUNK_SIZE, file_size - 1),
    };

    file.seek(SeekFrom::Start(range_start)).await.unwrap();
    let range_size = (range_end - range_start + 1) as usize;
    let mut data = vec![0u8; range_size];
    file.read_exact(&mut data).await.unwrap();

    let status_code = if range_end >= file_size {
        StatusCode::OK
    } else {
        StatusCode::PARTIAL_CONTENT
    };
    let response = Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, "video/webm") // TODO type should be dynamic from stored data
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, range_end - range_start + 1)
        .header(
            header::CONTENT_RANGE,
            format!("bytes {range_start}-{range_end}/{file_size}"),
        )
        .body(Full::from(data))
        .unwrap();
    Ok(response)
}
