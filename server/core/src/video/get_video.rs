use domain::{BytesPerSecond, FramesPerSecond, Id, Milliseconds, SizeBytes, Video};

pub struct DetailedVideo {
    pub id: Id,
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

impl From<Video> for DetailedVideo {
    fn from(value: Video) -> Self {
        DetailedVideo {
            id: value.id,
            name: value.name,
            short_desc: value.short_desc,
            long_desc: value.long_desc,
            sequent_id: value.sequent_id,
            size: value.size,
            duration: value.duration,
            bitrate: value.bitrate,
            resolution: value.resolution,
            framerate: value.framerate,
        }
    }
}

pub fn get_catalog(id: Id) -> DetailedVideo {
    todo!()
}
