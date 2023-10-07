use std::{
    fmt::Debug,
    io::{self, SeekFrom},
    path::{Path, PathBuf},
};

use ffprobe::{ffprobe, FfProbeError};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::{debug, instrument};

use crate::crud::metadata::CreateMetadataRequest;

#[derive(Debug)]
pub struct FileStore(pub PathBuf);

impl FileStore {
    #[instrument(skip(self))]
    pub async fn read_bytes<P: AsRef<Path> + Debug + Send>(
        &self,
        file_path: P,
        range_start: u64,
        range_end: u64,
    ) -> Result<Vec<u8>, io::Error> {
        let file_path = self.get_file(file_path);
        let mut file = match tokio::fs::File::open(file_path).await {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        file.seek(SeekFrom::Start(range_start)).await?;
        let range_size = usize::try_from(range_end - range_start + 1)
            .expect("while parsing range_size ->usize is outside of the range of u64");
        debug!("requested data size is {range_size} bytes");
        let mut data = vec![0u8; range_size];
        // TODO what if reaches end of file - if writing tests check the case
        // TODO what if too big of a range is requested - if writing tests check the case
        file.read_exact(&mut data).await?;
        Ok(data)
    }

    #[instrument]
    pub async fn get_metadata<P: AsRef<Path> + Debug + Send>(
        &self,
        file_path: P,
    ) -> Result<CreateMetadataRequest, FfProbeError> {
        let file_path = self.get_file(file_path);
        let ffprobe = tokio::task::spawn_blocking(move || ffprobe(file_path))
            .await
            .expect("error while spawning task")?;
        let streams = ffprobe.streams.get(0);

        let size = ffprobe.format.size.parse().unwrap_or(0);
        let duration = ffprobe
            .format
            .duration
            .map_or(0., |value| value.parse().unwrap_or(0.));
        let bitrate = ffprobe.format.bit_rate.unwrap_or_default();
        let width = streams.map_or_else(String::new, |value| value.width.unwrap_or(0).to_string());
        let height =
            streams.map_or_else(String::new, |value| value.height.unwrap_or(0).to_string());
        let framerate = streams
            .map_or_else(String::new, |value| value.avg_frame_rate.clone())
            .trim_end_matches("/1")
            .to_owned();

        Ok(CreateMetadataRequest {
            size,
            duration,
            bitrate,
            width,
            height,
            framerate,
        })
    }

    #[instrument(ret)]
    fn get_file<P: AsRef<Path> + Debug + Send>(&self, file_path: P) -> PathBuf {
        self.0.join(file_path)
    }
}
