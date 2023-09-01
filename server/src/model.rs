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

impl Catalog {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

pub type Bytes = i64;
pub type Seconds = i64;
pub type BytesPerSecond = i64;
pub type ScreenWidth = i16;
pub type ScreenHeight = i16;
pub type FramesPerSecond = f64;

#[derive(Debug, sqlx::FromRow)]
pub struct Video {
    pub id: EntityId,
    pub path: String,
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

impl Video {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{
        faker::{
            filesystem::en::{DirPath, FileName},
            lorem::en::{Paragraph, Word},
        },
        Fake, Faker,
    };
    use pretty_assertions::assert_eq;

    fn test_calalog() -> Catalog {
        Catalog {
            id: Faker.fake(),
            path: DirPath().fake(),
            display_name: Word().fake(),
            short_desc: Paragraph(1..2).fake(),
            long_desc: Paragraph(1..5).fake(),
            created_at: Faker.fake(),
            updated_at: Faker.fake(),
        }
    }

    fn test_video() -> Video {
        Video {
            id: Faker.fake(),
            path: FileName().fake(),
            display_name: Word().fake(),
            short_desc: Paragraph(1..2).fake(),
            long_desc: Paragraph(1..5).fake(),
            catalog_id: Faker.fake(),
            sequent_id: Faker.fake(),
            size: Faker.fake(),
            duration: Faker.fake(),
            bitrate: Faker.fake(),
            width: Faker.fake(),
            height: Faker.fake(),
            framerate: Faker.fake(),
            created_at: Faker.fake(),
            updated_at: Faker.fake(),
        }
    }

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
        let mut catalog = test_calalog();
        let path = test_path("/");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }

    #[test]
    fn path_catalog_backslash() {
        let mut catalog = test_calalog();
        let path = test_path("\\");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }

    #[test]
    fn path_video_slash() {
        let mut catalog = test_video();
        let path = test_path("/");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }

    #[test]
    fn path_video_backslash() {
        let mut catalog = test_video();
        let path = test_path("\\");
        catalog.path = path.clone();
        assert_eq!(catalog.path(), PathBuf::from(path));
    }
}
