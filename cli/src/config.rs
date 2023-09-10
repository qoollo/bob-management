use color_eyre::Result;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fs::File, io::BufReader, net::SocketAddr, path::PathBuf, time::Duration};
use tower_http::cors::CorsLayer;

/// Server Configuration passed on initialization
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Server address <host:port>
    pub address: SocketAddr,

    /// Enable Default Cors configuration
    #[serde(default = "default_cors")]
    pub cors_allow_all: bool,

    /// Max Time to Responce, in milliseconds
    #[serde(default = "default_timeout")]
    #[serde(with = "humantime_serde")]
    pub request_timeout: Duration,

    /// [`Logger`](LoggerConfig) Configuration
    #[serde(flatten)]
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
    #[serde(default = "default_log_amount")]
    pub log_amount: usize,

    /// [Stub] Max size of a single log file, in bytes
    #[serde(default = "default_log_size")]
    pub log_size: u64,

    /// Tracing Level
    #[serde(default = "tracing_default")]
    #[serde_as(as = "DisplayFromStr")]
    pub trace_level: tracing::Level,
}

impl Config {
    /// Return either very permissive [`CORS`](`CorsLayer`) configuration
    /// or empty one based on `cors_allow_all` field
    pub fn get_cors_configuration(&self) -> CorsLayer {
        self.cors_allow_all
            .then_some(CorsLayer::very_permissive())
            .unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 7000)),
            cors_allow_all: default_cors(),
            request_timeout: default_timeout(),
            logger: LoggerConfig::default(),
        }
    }
}

const fn tracing_default() -> tracing::Level {
    tracing::Level::INFO
}

const fn default_cors() -> bool {
    false
}

const fn default_timeout() -> Duration {
    Duration::from_millis(5000)
}

const fn default_log_amount() -> usize {
    5
}

const fn default_log_size() -> u64 {
    10u64.pow(6)
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: None,
            log_amount: default_log_amount(),
            log_size: default_log_size(),
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
    fn from_file(path: PathBuf) -> Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok((serde_yaml::from_reader(reader) as Result<Self, _>)?)
    }
}

impl FromFile for Config {}
impl FromFile for LoggerConfig {}
