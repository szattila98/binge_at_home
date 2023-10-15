use std::{
    fmt::Debug,
    io::{self, SeekFrom},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::bail;
use ffprobe::{ffprobe, FfProbeError};
use normpath::PathExt;
use notify::{Error, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{
    new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer, FileIdMap,
};
use sqlx::PgPool;
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    runtime::Handle,
    sync::mpsc::Receiver,
};
use tracing::{debug, error, info, instrument, warn, Instrument};

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

impl FileStore {
    pub fn new(config: &Configuration) -> Self {
        FileStore(config.file_store())
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

#[derive(Debug)]
pub struct StoreWatcher {
    file_store: Arc<FileStore>,
    pool: PgPool,
    debouncer: Option<Debouncer<RecommendedWatcher, FileIdMap>>,
    receiver: Option<Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>>,
}

impl StoreWatcher {
    pub async fn new(file_store: Arc<FileStore>, pool: PgPool) -> Self {
        let mut watcher = Self {
            file_store,
            pool,
            debouncer: None,
            receiver: None,
        };
        watcher.initialize_scheduler().await;
        watcher
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

            if let Some(mut rx) = self.receiver.take() {
                let pool = self.pool.clone();
                let file_store = self.file_store.clone();
                tokio::spawn(
                    async move {
                        while let Some(res) = rx.recv().await {
                            match res {
                                Ok(events) => {
                                    process_file_events(pool.clone(), file_store.clone(), events)
                                        .await
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

    #[instrument(skip_all)]
    async fn initialize_scheduler(&mut self) {
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        let rt = Handle::current();

        let debouncer = new_debouncer(
            Duration::from_secs(5),
            None,
            move |result: DebounceEventResult| {
                let tx = tx.clone();
                debug!("sending file event over channel");
                rt.spawn(
                    async move {
                        if let Err(e) = tx.send(result).await {
                            error!("error while sending file event: {:?}", e);
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
}

#[instrument(level = "debug", skip(pool, file_store))]
async fn process_file_events(
    pool: PgPool,
    file_store: Arc<FileStore>,
    events: Vec<DebouncedEvent>,
) {
    info!("processing {} file event(s)", events.len());
    let mut creations = vec![];
    let mut modifies = vec![];
    let mut removals = vec![];
    let mut others = vec![];
    for event in events {
        match event.kind {
            notify::EventKind::Create(_) => creations.push(event),
            notify::EventKind::Modify(_) => modifies.push(event),
            notify::EventKind::Remove(_) => removals.push(event),
            notify::EventKind::Access(_) | notify::EventKind::Any | notify::EventKind::Other => {
                others.push(event)
            }
        }
    }
    if !creations.is_empty() {
        let creations = creations
            .into_iter()
            .map(|event| event.paths.clone())
            .flatten()
            .map(|path| {
                path.normalize()
                    .expect("could not normalize path")
                    .into_path_buf()
            })
            .collect::<Vec<_>>();
        debug!("file creation events detected: \n{creations:#?}");
        // TODO make it in as few database operations as possible
        // TODO make easier to read, enum abstraction etc
        let mut tx = pool
            .begin()
            .await
            .expect("could not begin file saving transaction");
        for absolute_path in creations {
            let Ok(stripped_path) = absolute_path.strip_prefix(&file_store.0) else {
                error!(
                    "store prefix could not be stripped from path: {}",
                    absolute_path.display()
                );
                continue;
            };
            match stripped_path {
                stripped_path
                    if stripped_path.components().count() == 1 && absolute_path.is_dir() =>
                {
                    debug!(
                        "catalog detected, adding it to database with path '{}'",
                        stripped_path.display()
                    );
                    let catalog = CreateCatalogRequest::new(stripped_path.display().to_string());
                    if let Ok(catalog) = Catalog::create(&mut *tx, catalog).await {
                        info!("added catalog '{}'", catalog.path)
                    };
                    // TODO scan after creation to get all files under it in case they are not recorded to database somehow
                }
                stripped_path
                    if stripped_path.components().count() > 1 && absolute_path.is_file() =>
                {
                    let Some(catalog_path) = stripped_path.components().next() else {
                        error!(
                            "catalog part could not be extracted from path: {}",
                            absolute_path.display()
                        );
                        continue;
                    };
                    let catalog_path = catalog_path.as_os_str().to_string_lossy().to_string();
                    let Ok(opt) = Catalog::find_by_path(&mut *tx, catalog_path.clone()).await
                    else {
                        continue;
                    };
                    let parent_catalog_id = if let Some(catalog) = opt {
                        catalog.id
                    } else {
                        let catalog =
                            CreateCatalogRequest::new(stripped_path.display().to_string());
                        if let Ok(catalog) = Catalog::create(&mut *tx, catalog).await {
                            info!("added catalog '{}'", catalog.path);
                            catalog.id
                        } else {
                            continue;
                        }
                    };
                    let Ok(metadata) = file_store.get_metadata(&absolute_path).await else {
                        error!(
                            "could not read metadata for file: '{}'",
                            absolute_path.display()
                        );
                        continue;
                    };
                    let Ok(metadata) = Metadata::create(&mut *tx, metadata).await else {
                        error!(
                            "could not save metadata for file: '{}'",
                            absolute_path.display()
                        );
                        continue;
                    };
                    let video = CreateVideoRequest::new(
                        stripped_path.display().to_string(),
                        parent_catalog_id,
                        metadata.id,
                    );
                    if let Ok(video) = Video::create(&mut *tx, video).await {
                        info!("added catalog '{}'", video.path);
                    }
                }
                stripped_path
                    if stripped_path.components().count() == 1 && absolute_path.is_file() =>
                {
                    warn!(
                        "file added to root, not to a catalog, no action taken '{}'",
                        absolute_path.display()
                    );
                    continue;
                }
                _ => error!(
                    "could not determine what to do with path '{}'",
                    absolute_path.display()
                ),
            }
        }
        tx.commit()
            .await
            .expect("could not commit file saving transaction")
    }
    if !modifies.is_empty() {
        let modifies = modifies
            .into_iter()
            .map(|event| event.paths.clone())
            .flatten()
            .collect::<Vec<_>>();
        warn!("file modify events detected, no action taken: \n{modifies:#?}");
    }
    if !removals.is_empty() {
        let removals = removals
            .into_iter()
            .map(|event| event.paths.clone())
            .flatten()
            .collect::<Vec<_>>();
        warn!("file removal events detected, no action taken: \n{removals:#?}");
    }
    if !others.is_empty() {
        let others = others
            .into_iter()
            .map(|event| format!("{event:#?}"))
            .collect::<Vec<_>>()
            .join("\n");
        warn!("undefined file events detected, no action taken: \n{others}");
    }
}
