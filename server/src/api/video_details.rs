use askama::Template;
use axum::extract::State;
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
pub enum TemplateState {
    Ok { video: Video },
    VideoNotFound,
    DbErr(String),
}

#[derive(Serialize, Template)]
#[template(path = "video-details.html")]
pub struct PageTemplate {
    state: TemplateState,
}

impl PageTemplate {
    pub fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument]
pub async fn video_details(Endpoint { id }: Endpoint, State(pool): State<PgPool>) -> PageTemplate {
    let result = Video::find(&pool, id).await;
    let Ok(opt) = result else {
        return PageTemplate::new(TemplateState::DbErr(result.unwrap_err().to_string()));
    };
    let Some(video) = opt else {
        return PageTemplate::new(TemplateState::VideoNotFound);
    };

    let rendered = PageTemplate::new(TemplateState::Ok { video });
    debug!("video details rendered\n{rendered}");
    rendered
}
