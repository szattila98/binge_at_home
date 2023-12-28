use std::path::PathBuf;

#[cfg(test)]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};
use tracing_unwrap::ResultExt;

pub type EntityId = i64;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Catalog {
    pub id: EntityId,
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Catalog {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

pub type Bytes = i64;
pub type Seconds = f64;
pub type BytesPerSecond = String;
pub type ScreenWidth = String;
pub type ScreenHeight = String;
pub type FramesPerSecond = String;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, PartialEq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Metadata {
    pub id: EntityId,
    pub size: Bytes,
    pub duration: Seconds,
    pub bitrate: BytesPerSecond,
    pub width: ScreenWidth,
    pub height: ScreenHeight,
    pub framerate: FramesPerSecond,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Video {
    pub id: EntityId,
    pub path: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub catalog_id: EntityId,
    pub sequent_id: Option<EntityId>,
    pub metadata_id: Option<EntityId>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Video {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    pub fn extension(&self) -> String {
        self.path()
            .extension()
            .map_or_else(String::new, |s| s.to_string_lossy().to_string())
    }
}

static DATE_FORMAT: &[FormatItem] = format_description!("[year].[month].[day]. [hour]:[minute]");
pub trait FormatDate {
    fn format_date(&self, date: &OffsetDateTime) -> String {
        date.format(DATE_FORMAT).expect_or_log(
            "date formatting failed, it should not as format is compile time verified",
        )
    }
}

impl FormatDate for Catalog {}
impl FormatDate for Video {}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{faker::lorem::en::Word, Fake, Faker};
    use pretty_assertions::assert_eq;

    fn test_path(separator: &'static str) -> String {
        let path = format!(
            "{separator}{}{separator}{}{separator}{}",
            Word().fake::<String>(),
            Word().fake::<String>(),
            Word().fake::<String>(),
        );
        path
    }

    #[test]
    fn path_catalog_slash() {
        let mut catalog: Catalog = Faker.fake();
        let path = test_path("/");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }

    #[test]
    fn path_catalog_backslash() {
        let mut catalog: Catalog = Faker.fake();
        let path = test_path("\\");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }

    #[test]
    fn path_video_slash() {
        let mut video: Video = Faker.fake();
        let path = test_path("/");
        video.path = path.clone();
        assert_eq!(video.path(), PathBuf::from(path));
    }

    #[test]
    fn path_video_backslash() {
        let mut video: Video = Faker.fake();
        let path = test_path("\\");
        video.path = path.clone();
        assert_eq!(video.path(), PathBuf::from(path));
    }

    #[test]
    fn extension_video() {
        let mut video: Video = Faker.fake();
        let mut path = test_path("/");
        path.push_str(".test");
        video.path = path.clone();
        assert_eq!(video.extension(), "test".to_string());
    }

    #[test]
    fn extension_video_none() {
        let mut video: Video = Faker.fake();
        let path = test_path("/");
        video.path = path.clone();
        assert_eq!(video.extension(), "".to_string());
    }

    #[test]
    fn extension_video_empty() {
        let mut video: Video = Faker.fake();
        let mut path = test_path("/");
        path.push('.');
        video.path = path.clone();
        assert_eq!(video.extension(), "".to_string());
    }
}
