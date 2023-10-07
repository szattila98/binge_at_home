use std::path::PathBuf;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, instrument};

use crate::{
    crud::Entity,
    model::{Catalog, EntityId, Video},
};

use super::AppState;

#[derive(TypedPath, Deserialize)]
#[typed_path("/catalog/:catalog_id/browse/*path")]
pub struct Endpoint {
    catalog_id: EntityId,
    path: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum File {
    Directory { path: String, display_name: String },
    Video(Video),
}

#[derive(Serialize)]
enum TemplateState {
    Ok { catalog: Catalog, files: Vec<File> },
    CatalogNotFound,
    InvalidPath,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "browse.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    Endpoint { catalog_id, path }: Endpoint,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let (catalog_result, videos_result) = tokio::join!(
        Catalog::find(&pool, catalog_id),
        Video::find_by_catalog_id(&pool, catalog_id)
    );

    let catalog_opt = match catalog_result {
        Ok(catalog_opt) => catalog_opt,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HtmlTemplate::new(TemplateState::DbErr(e.to_string())),
            )
        }
    };
    let videos = match videos_result {
        Ok(videos) => videos,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HtmlTemplate::new(TemplateState::DbErr(e.to_string())),
            )
        }
    };

    let Some(catalog) = catalog_opt else {
        return (
            StatusCode::NOT_FOUND,
            HtmlTemplate::new(TemplateState::CatalogNotFound),
        );
    };

    let Some(files) = get_files(videos, PathBuf::from(path)) else {
        return (
            StatusCode::BAD_REQUEST,
            HtmlTemplate::new(TemplateState::InvalidPath),
        );
    };

    let rendered = HtmlTemplate::new(TemplateState::Ok { catalog, files });
    debug!("browse rendered\n{rendered}");
    (StatusCode::OK, rendered)
}

fn get_files(videos: Vec<Video>, walked_path: PathBuf) -> Option<Vec<File>> {
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
                let Some(display_name) = path.iter().next() else {
                    error!("path should never be empty, full path is '{video_path:?}' stripped_path is '{path:?}'");
                    return None;
                };
                let display_name = display_name.to_string_lossy().to_string();
                let path = format!("{}/{}", walked_path.display(), display_name).replace('\\', "/");
                Some(File::Directory { path, display_name })
            }
        })
        .collect::<Vec<_>>();
    // sorting and removing duplicates
    files.sort_by(|a, b| {
        use std::cmp::Ordering;
        match (a, b) {
            (File::Directory { path: dir1, .. }, File::Directory { path: dir2, .. }) => {
                dir1.cmp(dir2)
            }
            (File::Video(a), File::Video(b)) => a.display_name.cmp(&b.display_name),
            (File::Directory { .. }, File::Video(_)) => Ordering::Less,
            (File::Video(_), File::Directory { .. }) => Ordering::Greater,
        }
    });
    files.dedup();
    Some(files)
}
