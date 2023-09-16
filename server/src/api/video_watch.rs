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
#[typed_path("/video/:id/watch")]
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
#[template(path = "video-watch.html")]
pub struct HtmlTemplate {
    state: TemplateState,
}

impl HtmlTemplate {
    fn new(state: TemplateState) -> Self {
        Self { state }
    }
}

#[instrument(skip(pool))]
pub async fn video_watch(Endpoint { id }: Endpoint, State(pool): State<PgPool>) -> HtmlTemplate {
    let result = Video::find(&pool, id).await;
    let Ok(opt) = result else {
        return HtmlTemplate::new(TemplateState::DbErr(result.unwrap_err().to_string()));
    };
    let Some(video) = opt else {
        return HtmlTemplate::new(TemplateState::VideoNotFound);
    };

    let rendered = HtmlTemplate::new(TemplateState::Ok { video });
    debug!("list catalogs rendered\n{rendered}");
    rendered
}
