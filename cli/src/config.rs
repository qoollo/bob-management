use error_stack::{Context, Result, ResultExt};
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fmt::Display, fs::File, io::BufReader, net::SocketAddr, path::PathBuf, time::Duration};

/// Server Configuration passed on initialization
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Server address <host:port>
    pub address: SocketAddr,

    /// Enable Default Cors configuration
    #[serde(default = "Config::default_cors")]
    pub cors_allow_all: bool,

    /// Max Time to Responce, in milliseconds
    #[serde(default = "Config::default_timeout")]
    #[serde(with = "humantime_serde")]
    pub request_timeout: Duration,

    /// [`Logger`](LoggerConfig) Configuration
    #[serde(default)]
    pub logger: LoggerConfig,
}

/// Logger Configuration passed on initialization
#[allow(clippy::module_name_repetitions)]
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    /// [Stub] File to save logs
    pub log_file: Option<PathBuf>,

    /// [Stub] Number of log files
    #[serde(default = "LoggerConfig::default_log_amount")]
    pub log_amount: usize,

    /// [Stub] Max size of a single log file, in bytes
    #[serde(default = "LoggerConfig::default_log_size")]
    pub log_size: u64,

    /// Tracing Level
    #[serde(default = "LoggerConfig::tracing_default")]
    #[serde_as(as = "DisplayFromStr")]
    pub trace_level: tracing::Level,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 7000)),
            cors_allow_all: Self::default_cors(),
            request_timeout: Self::default_timeout(),
            logger: LoggerConfig::default(),
        }
    }
}

impl Config {
    pub const fn default_cors() -> bool {
        false
    }

    pub const fn default_timeout() -> Duration {
        Duration::from_millis(5000)
    }
}

impl LoggerConfig {
    pub const fn tracing_default() -> tracing::Level {
        tracing::Level::INFO
    }

    pub const fn default_log_amount() -> usize {
        5
    }

    pub const fn default_log_size() -> u64 {
        10u64.pow(6)
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: None,
            log_amount: Self::default_log_amount(),
            log_size: Self::default_log_size(),
            trace_level: tracing::Level::INFO,
        }
    }
}

pub trait FromFile {
    /// Parses the file spcified in `path`
    ///
    /// # Errors
    ///
    /// The fucntion will fail if either it couldn't open config file
    /// or failed to parse given file
    fn from_file(path: PathBuf) -> Result<Self, Error>
    where
        Self: Sized + DeserializeOwned,
    {
        let file = File::open(path).change_context(Error::FromFile)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).change_context(Error::FromFile)
    }
}

#[derive(Debug)]
pub enum Error {
    FromFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("configuration error: couldn't read from file")
    }
}

impl Context for Error {}

impl FromFile for Config {}
impl FromFile for LoggerConfig {}
