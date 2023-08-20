use std::path::PathBuf;

use time::OffsetDateTime;

pub type EntityId = i64;

#[derive(Debug, sqlx::FromRow)]
pub struct Catalog {
    pub id: EntityId,
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub type Bytes = u64;
pub type Seconds = u64;
pub type BytesPerSecond = u64;
pub type ScreenWidth = u16;
pub type ScreenHeight = u16;
pub type FramesPerSecond = f64;

#[derive(Debug, sqlx::FromRow)]
pub struct Video {
    pub id: EntityId,
    pub path: PathBuf,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,

    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
