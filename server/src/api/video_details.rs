use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tap::Tap;
use tracing::{debug, instrument};

use super::include::breadcrumbs::Breadcrumbs;
#[cfg(debug_assertions)]
use super::AppState;
use crate::{
    api::{
        include::breadcrumbs::extract_breadcrumbs, technical_error::redirect_to_technical_error,
    },
    configuration::Configuration,
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
}

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
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
    Endpoint { id }: Endpoint,
    State(config): State<Arc<Configuration>>,
    State(pool): State<PgPool>,
) -> Response {
    let opt = match Video::find(&pool, id).await {
        Ok(opt) => opt,
        Err(error) => {
            return redirect_to_technical_error(&config, error).into_response();
        }
    };
    let Some(video) = opt else {
        return HtmlTemplate::new(TemplateState::VideoNotFound).into_response();
    };

    let breadcrumbs = extract_breadcrumbs(video.catalog_id, &video.path);

    HtmlTemplate::new(TemplateState::Ok { video, breadcrumbs }).into_response()
}
