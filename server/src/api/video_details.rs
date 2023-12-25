use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, instrument};

use super::fragment::breadcrumbs::Breadcrumbs;
#[cfg(debug_assertions)]
use super::AppState;
use crate::{
    api::fragment::breadcrumbs::extract_breadcrumbs,
    crud::Entity,
    model::{EntityId, Video},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/video/:id")]
pub struct Endpoint {
    id: EntityId,
}

#[derive(Serialize)]
enum TemplateState {
    Ok {
        video: Video,
        breadcrumbs: Breadcrumbs,
    },
    VideoNotFound,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    pub const fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument]
#[axum_macros::debug_handler(state = AppState)]
pub async fn handler(Endpoint { id }: Endpoint, State(pool): State<PgPool>) -> impl IntoResponse {
    let opt = match Video::find(&pool, id).await {
        Ok(opt) => opt,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HtmlTemplate::new(TemplateState::DbErr(e.to_string())),
            )
        }
    };
    let Some(video) = opt else {
        return (
            StatusCode::NOT_FOUND,
            HtmlTemplate::new(TemplateState::VideoNotFound),
        );
    };

    let breadcrumbs = extract_breadcrumbs(video.catalog_id, &video.path);

    let rendered = HtmlTemplate::new(TemplateState::Ok { video, breadcrumbs });
    debug!("video details rendered\n{rendered}");
    (StatusCode::OK, rendered)
}
