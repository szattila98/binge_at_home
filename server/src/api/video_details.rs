use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, instrument};

use crate::{
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
    Ok { video: Video },
    VideoNotFound,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    pub fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument]
pub async fn handler(Endpoint { id }: Endpoint, State(pool): State<PgPool>) -> impl IntoResponse {
    let opt = match Video::find(&pool, id).await {
        Ok(opt) => opt,
        Err(e) => return HtmlTemplate::new(TemplateState::DbErr(e.to_string())),
    };
    let Some(video) = opt else {
        return HtmlTemplate::new(TemplateState::VideoNotFound);
    };

    let rendered = HtmlTemplate::new(TemplateState::Ok { video });
    debug!("video details rendered\n{rendered}");
    rendered
}
