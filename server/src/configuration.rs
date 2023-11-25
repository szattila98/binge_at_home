use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    net::IpAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{bail, Context};
use axum::http::HeaderValue;
use normpath::PathExt;
use secrecy::Secret;
use serde::Deserialize;

use confique::{
    yaml::{self, FormatOptions},
    Config,
};
use tower_http::cors::AllowOrigin;
use tracing::{info, instrument};

#[derive(Debug, Config, Clone)]
pub struct Configuration {
    /// Host to bind to.
    #[config(env = "HOST", default = "0.0.0.0")]
    host: IpAddr,
    /// Port to listen on.
    #[config(env = "PORT", default = 8080)]
    port: u16,
    /// Logging configuration options.
    #[config(nested)]
    logging: Logging,
    /// Database configuration options.
    #[config(nested)]
    database: Database,
    /// Server middleware configuration options.
    #[config(nested)]
    middlewares: Middlewares,
    /// Static directory to serve.
    #[config(env = "STATIC", default = "./templates/assets")]
    static_dir: String,
    /// File store configuration options.
    #[config(nested)]
    file_store: FileStore,
}

#[derive(Debug, Config, Deserialize, Clone)]
pub struct Logging {
    /// Log level. An integer between 1-5 or the level as a string.
    #[config(default = "info")]
    level: String,
    /// File logging configuration.
    #[config(nested)]
    file: LogFile,
}

#[derive(Debug, Config, Deserialize, Clone)]
pub struct LogFile {
    /// The parent directory of the log file.
    #[config(default = "logs")]
    dir: String,
    /// The name of the log file. A date will be appended to the end of it.
    #[config(default = "app.log")]
    name: String,
}

#[derive(Debug, Config, Deserialize, Clone)]
pub struct Database {
    /// The url of the postgres the data source.
    #[config(env = "DATABASE_URL")]
    url: Secret<String>,
}

#[derive(Debug, Config, Deserialize, Clone)]
pub struct Middlewares {
    /// The request body size limit in bytes.
    #[config(default = 4096)]
    body_size_limit: usize,
    /// The CORS policy for allowed origins.
    #[config(default = ["*"])]
    allowed_origins: Vec<String>,
    /// The number of seconds after the request handling will timeout.
    #[config(default = 30)]
    request_timeout: u64,
}

#[derive(Debug, Config, Deserialize, Clone)]
pub struct FileStore {
    /// The path of the file store root.
    #[config(env = "STORE")]
    path: String,
    /// The watcher debounce timeout, in seconds. The specified time will be awaited on file system event to filter duplicates.
    #[config(default = 3)]
    debounce_timeout: u64,
    /// The file system timeout in seconds. After a file system event is received, before processing the specified time will be awaited.
    #[config(default = 2)]
    fs_timeout: u64,
    /// The allowed extensions of videos.
    #[config(default = ["mp4", "webm"])]
    video_extensions: Vec<String>,
}

impl Configuration {
    #[instrument]
    pub fn load() -> anyhow::Result<Self> {
        let config_path = match env::var("BINGE_CONFIG_PATH") {
            Ok(path) => PathBuf::from(path),
            Err(e) => match e {
                env::VarError::NotPresent => PathBuf::from("./config/app.yml"),
                env::VarError::NotUnicode(path) => {
                    bail!("invalid CONFIG_PATH environmental variable, not unicode character found: '{path:?}'")
                }
            },
        };
        info!("loading configuration from {}", config_path.display());
        if !config_path.exists() {
            info!("creating configuration file template, make sure to provide required variables!");
            create_config_template(&config_path)?;
        }
        let config = Self::builder()
            .env()
            .file(&config_path)
            .load()
            .context("could not load configuration")
            .with_context(|| format!("config search path was: {}", config_path.display()))?;
        info!("loaded configuration");
        Ok(config)
    }

    pub const fn host(&self) -> IpAddr {
        self.host
    }

    pub const fn port(&self) -> u16 {
        self.port
    }

    pub const fn logging(&self) -> &Logging {
        &self.logging
    }

    pub const fn database(&self) -> &Database {
        &self.database
    }

    pub const fn middlewares(&self) -> &Middlewares {
        &self.middlewares
    }

    pub fn static_dir(&self) -> &str {
        self.static_dir.as_ref()
    }

    pub const fn file_store(&self) -> &FileStore {
        &self.file_store
    }
}

impl Logging {
    pub fn level(&self) -> &str {
        self.level.as_ref()
    }

    pub const fn file(&self) -> &LogFile {
        &self.file
    }
}

impl LogFile {
    pub fn dir(&self) -> &str {
        self.dir.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Database {
    pub const fn url(&self) -> &Secret<String> {
        &self.url
    }
}

impl Middlewares {
    const ANY_ORIGIN: &'static str = "*";

    pub const fn body_size_limit(&self) -> usize {
        self.body_size_limit
    }

    pub fn allowed_origins(&self) -> anyhow::Result<AllowOrigin> {
        if self.allowed_origins.contains(&Self::ANY_ORIGIN.to_string()) {
            return Ok(AllowOrigin::any());
        }

        let parsed: anyhow::Result<Vec<HeaderValue>> = self
            .allowed_origins
            .iter()
            .map(|origin| {
                HeaderValue::from_str(origin)
                    .map_err(|_| anyhow::anyhow!("provided origin '{origin}' cannot be parsed"))
            })
            .collect();

        Ok(AllowOrigin::list(parsed?))
    }

    pub const fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout)
    }
}

fn create_config_template(config_path: &PathBuf) -> Result<(), anyhow::Error> {
    let config_template = yaml::template::<Configuration>(FormatOptions::default());
    let parent_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
    create_dir_all(parent_dir)
        .with_context(|| format!("could not create directories: '{}'", parent_dir.display()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)
        .with_context(|| format!("failed to open or create file: '{}'", config_path.display()))?;
    file.write_all(config_template.as_bytes())
        .with_context(|| {
            format!(
                "failed to write config template into file: '{}'",
                config_path.display()
            )
        })?;
    Ok(())
}

impl FileStore {
    /// Returns the root path of this [`FileStore`].
    ///
    /// # Panics
    ///
    /// Panics if getting the current dir is not possible or if the file store path could not be normalized
    pub fn path(&self) -> PathBuf {
        let store = PathBuf::from(&self.path);
        if store.is_absolute() {
            store
        } else {
            env::current_dir()
                .expect("could not get current dir")
                .join(store)
                .normalize()
                .expect("could not normalize path")
                .into_path_buf()
        }
    }

    pub const fn debounce_timeout(&self) -> u64 {
        self.debounce_timeout
    }

    pub const fn fs_timeout(&self) -> u64 {
        self.fs_timeout
    }

    pub fn video_extensions(&self) -> &[String] {
        self.video_extensions.as_ref()
    }
}
