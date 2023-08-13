use std::path::PathBuf;

use time::{Duration, Instant};

pub type ModelId = u64;
pub type Bytes = u64;
pub type BytesPerSecond = u64;
pub type ScreenWidth = u16;
pub type ScreenHeight = u16;
pub type FramesPerSecond = f64;

#[derive(sqlx::FromRow)]
pub struct Catalog {
    pub id: ModelId,
    pub path: PathBuf,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,

    #[sqlx(skip)]
    pub videos: Vec<Video>,

    pub created_at: Instant,
    pub updated_at: Instant,
}

#[derive(sqlx::FromRow)]
pub struct Video {
    pub id: ModelId,
    pub path: PathBuf,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub sequent_id: ModelId,

    pub size: Bytes,
    pub duration: Duration,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,

    #[sqlx(skip)]
    pub tracks: Vec<Track>,

    pub created_at: Instant,
    pub updated_at: Instant,
}

#[derive(sqlx::FromRow)]
pub struct Track {
    pub lang: String,
    pub path: PathBuf,
}
