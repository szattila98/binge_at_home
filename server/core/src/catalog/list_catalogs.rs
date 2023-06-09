use domain::{Catalog, Id};

pub struct ListCatalogsRequest {
    pub page: u64,
    pub size: u64,
}

pub struct BriefCatalog {
    pub id: Id,
    pub name: String,
    pub short_desc: String,
    pub video_count: usize,
}

impl From<Catalog> for BriefCatalog {
    fn from(value: Catalog) -> Self {
        BriefCatalog {
            id: value.id,
            name: value.name,
            short_desc: value.short_desc,
            video_count: value.videos.len(),
        }
    }
}

pub fn list_catalogs(request: ListCatalogsRequest) -> Vec<BriefCatalog> {
    todo!()
}
