use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    net::IpAddr,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use serde::Deserialize;

use confique::{
    yaml::{self, FormatOptions},
    Config,
};

#[derive(Debug, Config)]
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
}

#[derive(Debug, Config, Deserialize)]
pub struct Logging {
    /// Log level. An integer between 1-5 or the level as a string.
    #[config(default = "info")]
    level: String,
    /// File logging configuration.
    #[config(nested)]
    file: LogFile,
}

#[derive(Debug, Config, Deserialize)]
pub struct LogFile {
    /// The parent directory of the log file.
    #[config(default = "logs")]
    dir: String,
    /// The name of the log file. A date will be appended to the end of it.
    #[config(default = "app.log")]
    name: String,
    /// A separate log file in the configured parent directory, that will have debug level logging. Useful for development.
    #[config(default = false)]
    separate_debug_file: bool,
}

impl Configuration {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = match env::var("BINGE_CONFIG_PATH") {
            Ok(path) => PathBuf::from(path),
            Err(e) => match e {
                env::VarError::NotPresent => PathBuf::from("./config/app.yml"),
                env::VarError::NotUnicode(path) => {
                    bail!("invalid CONFIG_PATH environmental variable: '{path:?}'")
                }
            },
        };
        if !config_path.exists() {
            create_config_template(&config_path)?;
        }
        let config = Self::builder()
            .env()
            .file(&config_path)
            .load()
            .context("could not load configuration")
            .with_context(|| format!("config search path was: {}", config_path.display()))?;
        Ok(config)
    }

    pub fn host(&self) -> IpAddr {
        self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn logging(&self) -> &Logging {
        &self.logging
    }
}

impl Logging {
    pub fn level(&self) -> &str {
        self.level.as_ref()
    }

    pub fn file(&self) -> &LogFile {
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

    pub fn separate_debug_file(&self) -> bool {
        self.separate_debug_file
    }
}

fn create_config_template(config_path: &PathBuf) -> Result<(), anyhow::Error> {
    let config_template = yaml::template::<Configuration>(FormatOptions::default());
    let parent_dir = config_path.parent().unwrap_or(Path::new("."));
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
