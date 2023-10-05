use std::{
    fmt::Debug,
    io::{self, SeekFrom},
    path::{Path, PathBuf},
};

use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct FileStore(pub PathBuf);

impl FileStore {
    #[instrument(skip(self))]
    pub async fn read_bytes<P: AsRef<Path> + Debug>(
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
        if let Err(e) = file.seek(SeekFrom::Start(range_start)).await {
            return Err(e);
        };
        let range_size = (range_end - range_start + 1) as usize;
        debug!("requested data size is {range_size} bytes");
        let mut data = vec![0u8; range_size];
        if let Err(e) = file.read_exact(&mut data).await {
            // TODO what if reaches end of file - if writing tests check the case
            // TODO what if too big of a range is requested - if writing tests check the case
            return Err(e);
        };
        return Ok(data);
    }

    #[instrument]
    pub fn get_metadata<P: AsRef<Path> + Debug>(&self, file_path: P) {}

    #[instrument(ret)]
    fn get_file<P: AsRef<Path> + Debug>(&self, file_path: P) -> PathBuf {
        self.0.join(file_path)
    }
}
