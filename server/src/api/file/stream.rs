use std::{ops::Bound, sync::Arc};

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
use tracing::instrument;

use crate::{
    api::AppState,
    crud::Entity,
    file_access::FileStore,
    model::{EntityId, Metadata, Video},
};

static MAX_CHUNK_SIZE: u64 = 1024 * 1024;

#[derive(TypedPath, Deserialize)]
#[typed_path("/video/:id/stream")]
pub struct Endpoint {
    id: EntityId,
}

#[instrument(skip(pool, file_store))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    Endpoint { id }: Endpoint,
    TypedHeader(range_header): TypedHeader<Range>,
    State(pool): State<PgPool>,
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

    let option = match Metadata::find(&pool, video.id).await {
        Ok(option) => option,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {e}"),
            ));
        }
    };
    let Some(metadata) = option else {
        return Err((
            StatusCode::NOT_FOUND,
            "metadata not found in database".to_string(),
        ));
    };

    assert!(
        metadata.size.is_positive(),
        "file size is listed as negative in the database"
    );
    let file_size = metadata.size.abs() as u64;

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

    let data = match file_store
        .read_bytes(&video.path, range_start, range_end)
        .await
    {
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("error while reading file: {e}"),
            ))
        }
        Ok(data) => data,
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
        .expect("error while building response");
    Ok(response)
}
