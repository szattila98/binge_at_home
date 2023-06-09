use domain::{Catalog, Id, Video};

pub struct BriefVideo {
    pub id: Id,
    pub name: String,
    pub short_desc: String,
}

impl From<Video> for BriefVideo {
    fn from(value: Video) -> Self {
        BriefVideo {
            id: value.id,
            name: value.name,
            short_desc: value.short_desc,
        }
    }
}

pub struct DetailedCatalog {
    pub id: Id,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub videos: Vec<BriefVideo>,
}

impl From<Catalog> for DetailedCatalog {
    fn from(value: Catalog) -> Self {
        DetailedCatalog {
            id: value.id,
            name: value.name,
            short_desc: value.short_desc,
            long_desc: value.long_desc,
            videos: value.videos.into_iter().map(|video| video.into()).collect(),
        }
    }
}

pub fn get_catalog(id: Id) -> DetailedCatalog {
    todo!()
}
