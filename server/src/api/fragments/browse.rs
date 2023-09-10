use std::path::PathBuf;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, instrument};

use crate::{
    crud::Entity,
    error::AppError,
    model::{Catalog, EntityId, Video},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/catalog/:catalog_id/browse/*path")]
pub struct BrowseEndpoint {
    catalog_id: EntityId,
    path: String,
}

#[derive(Debug, Serialize)]
enum Files {
    Directory(String),
    Video(Video),
}

#[derive(Serialize, Template)]
#[template(path = "fragments/browse.html")]
struct BrowseTemplate {
    catalog: Catalog,
    files: Vec<Files>,
}

#[instrument(skip(pool))]
pub async fn browse(
    BrowseEndpoint { catalog_id, path }: BrowseEndpoint,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError<sqlx::Error>> {
    // TODO better error, and empty video struct handling
    let catalog = Catalog::find(&pool, catalog_id)
        .await
        .map_err(|e| {
            error!("error while listing catalogs: {e}");
            e
        })?
        .unwrap();
    let videos = Video::find_by_catalog_id(&pool, catalog_id)
        .await
        .map_err(|e| {
            error!("error while listing catalogs: {e}");
            e
        })?;
    let files = get_files(videos, PathBuf::from(path));
    let rendered = BrowseTemplate { catalog, files };
    debug!("browse rendered\n{rendered}");
    Ok(rendered)
}

fn get_files(videos: Vec<Video>, path: PathBuf) -> Vec<Files> {
    // TODO better error handling, path too short, végignéz minden hiba lehetőség és kezel valahol, legyed debug assert
    debug_assert!(path.components().count() >= 1);
    let path = path.components().collect::<Vec<_>>()[1..]
        .iter()
        .collect::<PathBuf>(); // strip catalog part from path
    let videos = videos
        .into_iter()
        .filter(|video| video.path().starts_with(&path))
        .collect::<Vec<_>>();
    // ha 1 hosszú a path és extension van a végén akkor videó egyébként pedig az első része és folder
    todo!()
}
