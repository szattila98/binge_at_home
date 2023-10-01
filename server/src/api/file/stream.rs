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
use tracing::{debug, instrument};

use crate::{
    configuration::Configuration,
    crud::Entity,
    file_access::FileStore,
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
    State(file_store): State<Arc<FileStore>>,
) -> impl IntoResponse {
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

    let video_path = file_store.get_file(&video.path);
    let mut file = match tokio::fs::File::open(video_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("file could not be opened: {err}"),
            ))
        }
    };

    let file_size = file
        .metadata()
        .await
        .expect("could no read size of file")
        .len();

    let Some((range_start, range_end)) = range_header.iter().next() else {
        return Err((
            StatusCode::BAD_REQUEST,
            "range header should have at least one part specified".to_owned(),
        ));
    };
    let range_start = match range_start {
        Bound::Included(range_start) | Bound::Excluded(range_start) => range_start,
        Bound::Unbounded => 0,
    };
    let range_end = match range_end {
        Bound::Included(range_end) | Bound::Excluded(range_end) => range_end,
        Bound::Unbounded => u64::min(range_start + MAX_CHUNK_SIZE, file_size - 1),
    };

    if let Err(e) = file.seek(SeekFrom::Start(range_start)).await {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("invalid range start position, could not seek it in file: {e}"),
        ));
    };
    let range_size = (range_end - range_start + 1) as usize;
    debug!("requested data size is {range_size} bytes");
    let mut data = vec![0u8; range_size];
    if let Err(e) = file.read_exact(&mut data).await {
        // TODO what if reaches end of file - if writing tests check the case
        // TODO what if too big of a range is requested - if writing tests check the case
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error while reading file bytes: {e}"),
        ));
    };

    let status_code = if range_end >= file_size {
        // TODO this may not even happen because potential EOF error thrown by read exact - test it
        StatusCode::OK
    } else {
        StatusCode::PARTIAL_CONTENT
    };
    let response = Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, format!("video/{}", video.extension()))
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, range_end - range_start + 1)
        .header(
            header::CONTENT_RANGE,
            format!("bytes {range_start}-{range_end}/{file_size}"),
        )
        .body(Full::from(data))
        .expect("error whhile building response");
    Ok(response)
}
