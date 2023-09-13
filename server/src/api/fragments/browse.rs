use std::path::PathBuf;

use askama::Template;
use axum::{extract::State, response::IntoResponse, response::Response};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use tracing::{debug, error, instrument};

use crate::{
    crud::Entity,
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

#[derive(Error, Debug)]
pub enum BrowseError {
    #[error("catalog not found")]
    CatalogNotFound,
    #[error("invalid path")]
    InvalidPath,
    #[error("database error")]
    DbErr(#[from] sqlx::Error),
}

impl IntoResponse for BrowseError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        let status_code = match self {
            BrowseError::CatalogNotFound => StatusCode::NOT_FOUND,
            BrowseError::InvalidPath => StatusCode::NOT_FOUND,
            BrowseError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, msg).into_response()
    }
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
) -> Result<impl IntoResponse, BrowseError> {
    // TODO error handling into template - no need for pub - add to snippets
    let Some(catalog) = Catalog::find(&pool, catalog_id).await? else {
        return Err(BrowseError::CatalogNotFound);
    };
    let videos = Video::find_by_catalog_id(&pool, catalog_id).await?;
    let Some(files) = get_files(videos, PathBuf::from(path)) else {
        return Err(BrowseError::InvalidPath);
    };
    let rendered = BrowseTemplate { catalog, files };
    debug!("browse rendered\n{rendered}");
    Ok(rendered)
}

fn get_files(videos: Vec<Video>, walked_path: PathBuf) -> Option<Vec<File>> {
    // filtering videos starting with the walked_path
    let videos = videos
        .into_iter()
        .filter(|video| video.path().starts_with(&walked_path))
        .collect::<Vec<_>>();
    if videos.is_empty() {
        return None;
    };
    // collecting files at the current tree level
    let mut files = videos
        .into_iter()
        .filter_map(|video| {
            let video_path = video.path();
            let Ok(path) = video_path.strip_prefix(&walked_path) else {
                error!("prefix of path '{video_path:?}' was not walked path '{walked_path:?}'");
                return None;
            };
            let path = path.to_path_buf();

            if path.components().count() == 1 && path.extension().map_or(false, |ext| ext == "webm")
            {
                Some(File::Video(video))
            } else {
                let Some(path) = path.iter().next() else {
                    error!("path should never be empty, full path is '{video_path:?}' stripped_path is '{path:?}'");
                    return None;
                };
                Some(File::Directory(path.to_string_lossy().to_string()))
            }
        })
        .collect::<Vec<_>>();
    // sorting and removing duplicates
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
    Some(files)
}
