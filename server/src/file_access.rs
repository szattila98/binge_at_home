use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, SeekFrom},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::bail;
use ffprobe::{ffprobe, FfProbeError};
use notify::{Error, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use serde::Serialize;
use sqlx::PgPool;
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    runtime::Handle,
    sync::mpsc::Receiver,
};
use tracing::{debug, error, info, instrument, warn, Instrument};
use walkdir::{DirEntry, WalkDir};

use crate::{
    configuration::Configuration,
    crud::{
        catalog::CreateCatalogRequest, metadata::CreateMetadataRequest, video::CreateVideoRequest,
        Entity,
    },
    model::{Catalog, Metadata, Video},
};

#[derive(Debug)]
pub struct FileStore(PathBuf);

#[derive(Debug, Default, Serialize)]
pub struct FileStoreChanges {
    added_catalogs: usize,
    added_videos: usize,
}

impl FileStore {
    pub fn new(config: &Configuration) -> Self {
        FileStore(config.file_store().path())
    }

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
    async fn get_metadata<P: AsRef<Path> + Debug + Send>(
        &self,
        file_path: P,
    ) -> Result<CreateMetadataRequest, FfProbeError> {
        let file_path = self.get_file(file_path);
        let ffprobe = tokio::task::spawn_blocking(move || ffprobe(file_path))
            .await
            .expect("error while spawning task")
            .map_err(|e| {
                error!("error while getting metadata: {e}");
                e
            })?;
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

    #[instrument]
    fn get_file<P: AsRef<Path> + Debug + Send>(&self, file_path: P) -> PathBuf {
        self.0.join(file_path)
    }

    #[instrument(skip(self, pool))]
    pub async fn scan_store_and_track_changes(
        &self,
        pool: PgPool,
    ) -> anyhow::Result<FileStoreChanges> {
        info!("file processing started...");
        let Ok(mut tx) = pool.begin().await.map_err(|e| {
            error!("could not begin file watcher transaction: {e}");
            e
        }) else {
            bail!("database error");
        };

        let Ok(db_catalogs) = Catalog::find_all(&mut *tx, vec![], None).await else {
            bail!("database error");
        };
        let Ok(db_videos) = Video::find_all(&mut *tx, vec![], None).await else {
            bail!("database error");
        };
        let db_catalogs = db_catalogs
            .into_iter()
            .map(|catalog| catalog.path())
            .collect::<HashSet<_>>();
        let db_videos = db_videos
            .into_iter()
            .map(|catalog| catalog.path())
            .collect::<HashSet<_>>();
        debug!(
            "catalogs in database: {} | videos in database: {}",
            db_catalogs.len(),
            db_videos.len(),
        );

        let (fs_catalogs, fs_videos): (HashSet<_>, HashSet<_>) = WalkDir::new(&self.0)
            .follow_root_links(false)
            .into_iter()
            .filter_map(|result| match result {
                Ok(entry) => Some(entry),
                Err(error) => {
                    error!("walkdir error: {:?}", error);
                    None
                }
            })
            .filter_map(|entry| is_file_in_catalog_or_catalog(entry, &self.0))
            .map(|entry| entry.path().to_path_buf())
            .partition(|path| path.is_dir());
        let fs_catalogs = fs_catalogs
            .into_iter()
            .map(|path| path.strip_prefix(&self.0).unwrap().to_path_buf())
            .collect::<HashSet<_>>();
        let fs_videos = fs_videos
            .into_iter()
            .map(|path| path.strip_prefix(&self.0).unwrap().to_path_buf())
            .collect::<HashSet<_>>();
        debug!(
            "catalogs on file system {} | videos on file system: {}",
            fs_catalogs.len(),
            fs_videos.len()
        );

        let catalogs_not_in_db = fs_catalogs.difference(&db_catalogs).collect::<Vec<_>>();
        debug!("catalogs not in database: {catalogs_not_in_db:#?}");
        let videos_not_in_db = fs_videos.difference(&db_videos).collect::<Vec<_>>();
        debug!("videos not in database: {videos_not_in_db:#?}");

        if catalogs_not_in_db.is_empty() && videos_not_in_db.is_empty() {
            info!("no catalogs or videos added to file store, no actions taken");
            return Ok(FileStoreChanges::default());
        }

        let requests = catalogs_not_in_db
            .into_iter()
            .map(|path| CreateCatalogRequest::new(path.to_string_lossy().to_string()))
            .collect();
        let Ok(catalogs) = Catalog::create_many(&mut *tx, requests).await else {
            bail!("database error");
        };
        let added_catalogs = catalogs.len();
        info!("{} catalogs added to the store", added_catalogs);

        let mut added_videos = 0;
        for path in videos_not_in_db {
            let catalog_path = path
                .components()
                .next()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string();
            let Ok(catalog) = Catalog::find_by_path(&mut *tx, &catalog_path).await else {
                continue;
            };
            let Some(catalog) = catalog else {
                warn!(
                    "parent catalog not found in database: {catalog_path} - {}",
                    path.display()
                );
                continue;
            };
            let metadata_id = match self.get_metadata(&path).await {
                Ok(request) => match Metadata::create(&mut *tx, request).await {
                    Ok(metadata) => Some(metadata.id),
                    Err(_) => None,
                },
                Err(_) => None,
            };

            let request = CreateVideoRequest::new(
                path.to_string_lossy().to_string(),
                catalog.id,
                metadata_id,
            );
            let Ok(_) = Video::create(&mut *tx, request).await else {
                continue;
            };
            added_videos += 1;
        }
        info!("{added_videos} video(s) added to the database");

        let _ = tx
            .commit()
            .await
            .map_err(|e| error!("could not commit file watcher transaction: {e}"));
        info!("finished processing new files");

        return Ok(FileStoreChanges {
            added_catalogs,
            added_videos,
        });
    }
}

#[instrument(skip(entry, store))]
fn is_file_in_catalog_or_catalog(entry: DirEntry, store: &Path) -> Option<DirEntry> {
    let file_type = entry.file_type();
    let path = entry.path().strip_prefix(store).unwrap();

    let is_file_in_catalog = file_type.is_file() && {
        let is_in_catalog = path.components().count() > 1;
        (!is_in_catalog).then(|| {
            warn!(
                "file is in root, not in catalog, it will be ignored: '{}'",
                path.display()
            )
        });
        is_in_catalog
    };

    let is_file_in_catalog_or_catalog = is_file_in_catalog || {
        let is_dir = entry.file_type().is_dir();
        let is_catalog = path.components().count() == 1;
        is_dir && is_catalog
    };

    is_file_in_catalog_or_catalog.then_some(entry)
}

#[derive(Debug)]
pub struct StoreWatcher {
    config: Configuration,
    file_store: Arc<FileStore>,
    pool: PgPool,
    debouncer: Option<Debouncer<RecommendedWatcher, FileIdMap>>,
    receiver: Option<Receiver<Result<usize, Vec<Error>>>>,
}

impl StoreWatcher {
    pub async fn new(config: Configuration, file_store: Arc<FileStore>, pool: PgPool) -> Self {
        let mut watcher = Self {
            config,
            file_store,
            pool,
            debouncer: None,
            receiver: None,
        };
        watcher.initialize_scheduler().await;
        watcher
    }

    #[instrument(skip_all)]
    async fn initialize_scheduler(&mut self) {
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        let rt = Handle::current();

        let debouncer = new_debouncer(
            Duration::from_secs(self.config.file_store().debounce_timeout()),
            None,
            move |result: DebounceEventResult| {
                let tx = tx.clone();
                let result = match result {
                    Ok(events) => Ok(events.len()),
                    Err(errors) => Err(errors),
                };
                debug!("sending file event over channel\n{result:#?}");
                rt.spawn(
                    async move {
                        if let Err(e) = tx.send(result).await {
                            error!("error while sending file event: {e:?}");
                        }
                    }
                    .in_current_span(),
                );
            },
        );

        match debouncer {
            Ok(watcher) => {
                info!("file watcher initialized");
                self.debouncer = Some(watcher);
                self.receiver = Some(rx);
            }
            Err(error) => {
                error!("error while initializing watcher: {:?}", error);
            }
        }
    }

    #[instrument(skip_all)]
    pub fn watch_store(&mut self) -> anyhow::Result<()> {
        let watch_path = &self.file_store.0;

        if watch_path.exists() {
            info!("watching store: '{}'", watch_path.display());
        } else {
            bail!("store that should be watched does not exist");
        }

        if let Some(debouncer) = self.debouncer.as_mut() {
            debouncer
                .watcher()
                .watch(watch_path, RecursiveMode::Recursive)?;
            debouncer
                .cache()
                .add_root(watch_path, RecursiveMode::Recursive);

            let timeout = self.config.file_store().fs_timeout();
            if let Some(mut rx) = self.receiver.take() {
                let pool = self.pool.clone();
                let file_store = self.file_store.clone();
                tokio::spawn(
                    async move {
                        while let Some(res) = rx.recv().await {
                            match res {
                                Ok(change_count) => {
                                    info!("detected {change_count} change(s), waiting {timeout} seconds for file changes to be written to disk");
                                    tokio::time::sleep(Duration::from_secs(timeout)).await;
                                    let _ = file_store.scan_store_and_track_changes(pool.clone()).await;
                                }
                                Err(errors) => {
                                    error!(
                                        "notify error(s) detected: {}",
                                        errors
                                            .iter()
                                            .map(|error| error.to_string())
                                            .collect::<Vec<_>>()
                                            .join(" | ")
                                    );
                                }
                            }
                        }
                    }
                    .in_current_span(),
                );
            } else {
                bail!("store watcher receiver not initialized");
            }
        } else {
            bail!("store watcher debouncer not initialized")
        }

        Ok(())
    }
}
