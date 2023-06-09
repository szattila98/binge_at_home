pub type Id = u64;
pub type SizeBytes = u64;
pub type Milliseconds = u64;
pub type BytesPerSecond = u64;
pub type FramesPerSecond = f64;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Catalog {
    pub id: Id,
    pub dir: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub videos: Vec<Video>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Video {
    pub id: Id,
    pub path: String,
    pub name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub sequent_id: Id,
    pub size: SizeBytes,
    pub duration: Milliseconds,
    pub bitrate: BytesPerSecond,
    pub resolution: String,
    pub framerate: FramesPerSecond,
}
