use std::path::PathBuf;

use askama::Template;
use axum::extract::State;
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, instrument};

use crate::{
    crud::Entity,
    model::{Catalog, EntityId, Video},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/catalog/:catalog_id/browse/*path")]
pub struct Endpoint {
    catalog_id: EntityId,
    path: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum File {
    Directory(String),
    Video(Video),
}

#[derive(Serialize)]
pub enum TemplateState {
    Ok { catalog: Catalog, files: Vec<File> },
    CatalogNotFound,
    InvalidPath,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "fragments/browse.html")]
pub struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
pub async fn browse(
    Endpoint { catalog_id, path }: Endpoint,
    State(pool): State<PgPool>,
) -> HtmlTemplate {
    // TODO add to snippets
    let (catalog_result, videos_result) = tokio::join!(
        Catalog::find(&pool, catalog_id),
        Video::find_by_catalog_id(&pool, catalog_id)
    );

    let Ok(catalog_opt) = catalog_result else {
        return HtmlTemplate::new(TemplateState::DbErr(
            catalog_result.unwrap_err().to_string(),
        ));
    };
    let Ok(videos) = videos_result else {
        return HtmlTemplate::new(TemplateState::DbErr(videos_result.unwrap_err().to_string()));
    };

    let Some(catalog) = catalog_opt else {
        return HtmlTemplate::new(TemplateState::CatalogNotFound);
    };

    let Some(files) = get_files(videos, PathBuf::from(path)) else {
        return HtmlTemplate::new(TemplateState::InvalidPath);
    };

    let rendered = HtmlTemplate::new(TemplateState::Ok { catalog, files });
    debug!("browse rendered\n{rendered}");
    rendered
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
