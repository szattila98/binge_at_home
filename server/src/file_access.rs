use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileStore(pub PathBuf);

impl FileStore {
    pub fn get_file<P: AsRef<Path>>(&self, file_path: P) -> Option<PathBuf> {
        let file = self.0.join(file_path);
        file.exists().then(|| file)
    }
}
