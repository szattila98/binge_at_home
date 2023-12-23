use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::routing::TypedPath;
use sqlx::PgPool;
use tracing::instrument;

use crate::file_access::FileStore;

#[cfg(debug_assertions)]
use crate::api::AppState;

#[derive(TypedPath)]
#[typed_path("/store/scan")]
pub struct Endpoint;

#[instrument(skip(pool, file_store))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    _: Endpoint,
    State(pool): State<PgPool>,
    State(file_store): State<Arc<FileStore>>,
) -> impl IntoResponse {
    match file_store.scan_store_and_track_changes(pool).await {
        Ok(changes) => Ok(Json(changes)),
        Err(error) => Err(Json(serde_json::json!({
            "error_msg": error.to_string()
        }))),
    }
}
