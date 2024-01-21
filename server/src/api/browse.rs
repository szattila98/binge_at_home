use std::{path::PathBuf, sync::Arc};

use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tap::Tap;
use tracing::{debug, error, instrument};

use super::include::breadcrumbs::BreadcrumbsTemplate;
#[cfg(debug_assertions)]
use super::AppState;

use crate::{
    api::technical_error::redirect_to_technical_error,
    configuration::Configuration,
    crud::Entity,
    model::{Catalog, EntityId, Video},
};

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
    Ok {
        catalog: Catalog,
        files: Vec<File>,
        breadcrumbs: BreadcrumbsTemplate,
    },
    CatalogNotFound,
    InvalidPath,
}

#[derive(Serialize, Template)]
#[template(path = "browse.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }.tap(|template| debug!("rendered html template:\n{template}"))
    }
}

#[instrument(skip(config, pool))]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(
    Endpoint { catalog_id, path }: Endpoint,
    State(config): State<Arc<Configuration>>,
    State(pool): State<PgPool>,
) -> Response {
    let (catalog_result, videos_result) = tokio::join!(
        Catalog::find(&pool, catalog_id),
        Video::find_by_catalog_id(&pool, catalog_id)
    );

    let catalog_opt = match catalog_result {
        Ok(catalog_opt) => catalog_opt,
        Err(error) => {
            return redirect_to_technical_error(&config, &error).into_response();
        }
    };
    let videos = match videos_result {
        Ok(videos) => videos,
        Err(error) => {
            return redirect_to_technical_error(&config, &error).into_response();
        }
    };

    let Some(catalog) = catalog_opt else {
        return HtmlTemplate::new(TemplateState::CatalogNotFound).into_response();
    };

    let breadcrumbs = BreadcrumbsTemplate::new(catalog.id, &path);

    let Some(files) = get_files(videos, &PathBuf::from(path)) else {
        return HtmlTemplate::new(TemplateState::InvalidPath).into_response();
    };

    HtmlTemplate::new(TemplateState::Ok {
        catalog,
        files,
        breadcrumbs,
    })
    .into_response()
}

fn get_files(videos: Vec<Video>, walked_path: &PathBuf) -> Option<Vec<File>> {
    let videos = videos
        .into_iter()
        .filter(|video| video.path().starts_with(walked_path))
        .collect::<Vec<_>>();
    if videos.is_empty() {
        return None;
    };
    // collecting files at the current tree level
    let mut files = videos
        .into_iter()
        .filter_map(|video| {
            let video_path = video.path();
            let Ok(stripped_path) = video_path.strip_prefix(walked_path) else {
                error!("prefix of path '{video_path:?}' was not walked path '{walked_path:?}'");
                return None;
            };
            let stripped_path = stripped_path.to_path_buf();

            if stripped_path.components().count() == 1 && stripped_path.extension().map_or(false, |ext| ext == "webm")
            {
                Some(File::Video(video))
            } else {
                let Some(display_name) = stripped_path.iter().next() else {
                    error!("path should never be empty, full path is '{video_path:?}' stripped_path is '{stripped_path:?}'");
                    return None;
                };
                let display_name = display_name.to_string_lossy().to_string();
                let dir_path = format!("{}/{}", walked_path.display(), display_name).replace('\\', "/");
                Some(File::Directory { path: dir_path, display_name })
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
