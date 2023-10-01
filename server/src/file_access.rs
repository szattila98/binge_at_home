use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use tracing::instrument;

#[derive(Debug)]
pub struct FileStore(pub PathBuf);

impl FileStore {
    #[instrument]
    pub fn get_file<P: AsRef<Path> + Debug>(&self, file_path: P) -> PathBuf {
        self.0.join(file_path)
    }

    #[instrument]
    pub fn get_metadata<P: AsRef<Path> + Debug>(&self, file_path: P) {}
}
