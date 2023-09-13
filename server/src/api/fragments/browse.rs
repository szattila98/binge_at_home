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

#[derive(Debug, Serialize, PartialEq)]
enum File {
    Directory(String),
    Video(Video),
}

#[derive(Serialize, Template)]
#[template(path = "fragments/browse.html")]
struct BrowseTemplate {
    catalog: Catalog,
    files: Vec<File>,
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

fn get_files(videos: Vec<Video>, walked_path: PathBuf) -> Vec<File> {
    // TODO legyen debug assert és debug log ha valami None
    // TODO windows esetén is jó-e
    debug_assert!(walked_path.components().count() >= 1);

    // stripping catalog name
    let walked_path = walked_path.components().collect::<Vec<_>>()[1..]
        .iter()
        .collect::<PathBuf>();
    // filtering videos starting with the walked_path
    let videos = videos
        .into_iter()
        .filter(|video| video.path().starts_with(&walked_path))
        .collect::<Vec<_>>();
    // collecting files at the current tree level
    let mut files = videos
        .into_iter()
        .filter_map(|video| {
            let video_path = video.path();
            let Ok(path) = video_path.strip_prefix(&walked_path) else {
                return None;
            };
            let path = path.to_path_buf();

            if path.components().count() == 1 && path.extension().map_or(false, |ext| ext == "webm")
            {
                Some(File::Video(video))
            } else {
                let Some(path) = path.iter().next() else {
                    error!("path should never be empty, full path = {video_path:?} stripped_path = {path:?}");
                    return None;
                };
                Some(File::Directory(path.to_string_lossy().to_string()))
            }
        })
        .collect::<Vec<_>>();
    files.sort_by(|a, b| {
        use std::cmp::Ordering;
        match (a, b) {
            (File::Directory(dir1), File::Directory(dir2)) => dir1.cmp(dir2),
            (File::Video(video1), File::Video(video2)) => {
                video1.display_name.cmp(&video2.display_name)
            }
            (File::Directory(_), File::Video(_)) => Ordering::Less,
            (File::Video(_), File::Directory(_)) => Ordering::Greater,
        }
    });
    files.dedup();
    files
}
