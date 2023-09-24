use std::{io::SeekFrom, ops::Bound, sync::Arc};

use axum::{
    body::{Body, Bytes, StreamBody},
    extract::State,
    headers::Range,
    response::{AppendHeaders, IntoResponse, Response},
    TypedHeader,
};
use axum_extra::routing::TypedPath;
use http::{header, HeaderMap, StatusCode};
use http_body::Full;
use serde::Deserialize;
use sqlx::PgPool;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
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
    headers: HeaderMap,
    //TypedHeader(range): TypedHeader<Range>,
    State(pool): State<PgPool>,
    State(config): State<Arc<Configuration>>,
) -> Result<Response<Full<Bytes>>, impl IntoResponse> {
    // TODO better error handling
    // TODO type should be dynamic
    // TODO refactor
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

    let mut video_path = config.store();
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

    static CHUNK_SIZE: u64 = 314700;

    let mut range_start = 0;
    let mut range_end = CHUNK_SIZE;
    let file_size = file.metadata().await.unwrap().len();

    let Some(range) = headers.get("range") else {
        file.seek(SeekFrom::Start(range_start)).await.unwrap();
        let range_size = (range_end - range_start + 1) as usize;
        let mut buffer = vec![0u8; range_size];
        file.read_exact(&mut buffer).await.unwrap();

        let response = Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header("Content-Type", "video/webm")
            .header("Accept-Ranges", "bytes")
            .header("Content-Length", range_end)
            .header(
                "Content-Range",
                format!("bytes {range_start}-{range_end}/{file_size}"),
            )
            .header("Content-Length", file_size)
            .body(Full::from(buffer))
            .unwrap();
        return Ok(response);
    };

    let ranges = range
        .to_str()
        .unwrap()
        .trim_start_matches("bytes=")
        .split('-')
        .collect::<Vec<_>>();
    range_start = ranges[0].parse().unwrap();
    if ranges.len() > 1 {
        range_end = ranges[1].parse().unwrap_or(range_start + CHUNK_SIZE);
    } else {
        range_end = range_start + CHUNK_SIZE;
    }
    range_end = u64::min(range_end, file_size - 1);

    file.seek(SeekFrom::Start(range_start)).await.unwrap();
    let range_size = (range_end - range_start + 1) as usize;
    let mut data = vec![0u8; range_size];
    file.read_exact(&mut data).await.unwrap();

    let content_length = range_end - range_start + 1;
    let status = if range_end >= file_size {
        StatusCode::OK
    } else {
        StatusCode::PARTIAL_CONTENT
    };

    let response = Response::builder()
        .status(status)
        .header("Content-Type", "video/webm")
        .header("Accept-Ranges", "bytes")
        .header("Content-Length", content_length)
        .header(
            "Content-Range",
            format!("bytes {range_start}-{range_end}/{file_size}"),
        )
        .body(Full::from(data))
        .unwrap();
    Ok(response)
}
