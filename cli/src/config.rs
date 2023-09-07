use color_eyre::Result;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fs::File, io::BufReader, net::SocketAddr, path::PathBuf};
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, Deserialize)]
pub struct Timeout(u64);
#[derive(Debug, Clone, Deserialize)]
pub struct CorsAllowAll(bool);
#[derive(Debug, Clone, Deserialize)]
pub struct LogAmount(usize);
#[derive(Debug, Clone, Deserialize)]
pub struct LogSize(u64);

/// Server Configuration passed on initialization
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Server address <host:port>
    pub address: SocketAddr,

    /// Enable Default Cors configuration
    #[serde(default)]
    pub cors: CorsAllowAll,

    /// Max Time to Responce, in milliseconds
    #[serde(default)]
    pub request_timeout: Timeout,
}

/// Logger Configuration passed on initialization
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Logger {
    /// [Stub] File to save logs
    pub log_file: Option<PathBuf>,

    /// [Stub] Number of log files
    #[serde(default)]
    pub log_amount: LogAmount,

    /// [Stub] Max size of a single log file, in bytes
    #[serde(default)]
    pub log_size: LogSize,

    /// Tracing Level
    #[serde(default = "tracing_default")]
    #[serde_as(as = "DisplayFromStr")]
    pub trace_level: tracing::Level,
}

const fn tracing_default() -> tracing::Level {
    tracing::Level::INFO
}

#[allow(clippy::derivable_impls)]
impl Default for CorsAllowAll {
    fn default() -> Self {
        Self(false)
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self(5000)
    }
}

impl Default for LogAmount {
    fn default() -> Self {
        Self(5)
    }
}

impl Default for LogSize {
    fn default() -> Self {
        Self(10u64.pow(6))
    }
}

impl From<CorsAllowAll> for CorsLayer {
    fn from(value: CorsAllowAll) -> Self {
        value
            .0
            .then_some(Self::very_permissive())
            .unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 7000)),
            cors: CorsAllowAll::default(),
            request_timeout: Timeout::default(),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            log_file: None,
            log_amount: LogAmount::default(),
            log_size: LogSize::default(),
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
impl FromFile for Logger {}
