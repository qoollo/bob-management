use color_eyre::Result;
use hyper::{http::HeaderName, Method};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::{fs::File, io::BufReader, net::SocketAddr, path::PathBuf, time::Duration};
use tower_http::cors::CorsLayer;

pub type Timeout = u64;

/// Server Configuration passed on initialization
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Server address <host:port>
    pub address: SocketAddr,

    /// Cors configuration
    pub cors: Cors,

    /// Max Time to Responce, in milliseconds
    pub request_timeout: Timeout,

    /// [Stub] File to save logs
    pub log_file: Option<PathBuf>,

    /// Tracing Level
    #[serde(default = "tracing_default")]
    #[serde_as(as = "DisplayFromStr")]
    pub trace_level: tracing::Level,
}

const fn tracing_default() -> tracing::Level {
    tracing::Level::INFO
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 6000)),
            log_file: None,
            trace_level: tracing::Level::INFO,
            cors: Cors::default(),
            request_timeout: 5000,
        }
    }
}

impl Config {
    /// Parses the file spcified in `path`
    ///
    /// # Errors
    ///
    /// The fucntion will fail if either it couldn't open config file
    /// or failed to parse given file
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok((serde_yaml::from_reader(reader) as Result<Self, _>)?)
    }
}

/// Cors Configuration struct
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cors {
    /// The `Access-Control-Allow-Origin` response header
    ///
    /// Indicates whether the response can be shared with requesting code from the given origins.
    pub origins: Vec<String>,

    /// The `Access-Control-Allow-Credentials` response header
    ///
    /// Tells browsers whether to expose the response to the frontend JavaScript code when the request's credentials mode (`Request.credentials`) is `include`
    pub allow_credentials: Option<bool>,

    /// The `Access-Control-Allow-Credentials` response header
    ///
    /// Tells browsers whether to expose the response to the frontend JavaScript code when the request's credentials mode (`Request.credentials`) is `include`
    pub allow_methods: Vec<String>,

    /// The `Access-Control-Allow-Private-Network` response header
    ///
    /// Indicates that a resource can be safely shared with external networks.
    pub allow_private_network: Option<bool>,

    /// The `Access-Control-Allow-Headers` response header
    ///
    /// Used in response to a preflight request
    /// which includes the Access-Control-Request-Headers
    /// to indicate which HTTP headers can be used during the actual request.    
    pub allow_headers: Vec<String>,

    /// The Access-Control-Expose-Headers response header
    ///
    /// Allows a server to indicate which
    /// response headers should be made available to scripts running in the browser,
    /// in response to a cross-origin request.
    pub expose_headers: Option<Vec<String>>,

    /// The `Access-Control-Max-Age` response header
    ///
    /// Indicates how long the results of a preflight request can be cached, in seconds.
    pub max_age: Option<u64>,
}

impl From<Cors> for CorsLayer {
    fn from(value: Cors) -> Self {
        Self::new()
            .allow_methods(
                value
                    .allow_methods
                    .iter()
                    .filter_map(|method| method.parse::<Method>().ok())
                    .collect::<Vec<_>>(),
            )
            .allow_credentials(value.allow_credentials.unwrap_or_default())
            .allow_private_network(value.allow_private_network.unwrap_or_default())
            .allow_headers(
                value
                    .allow_headers
                    .iter()
                    .filter_map(|header| header.parse::<HeaderName>().ok())
                    .collect::<Vec<_>>(),
            )
            .allow_origin(
                value
                    .origins
                    .iter()
                    .filter_map(|orig| orig.parse().ok())
                    .collect::<Vec<_>>(),
            )
            .expose_headers(
                value
                    .expose_headers
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|header| header.parse::<HeaderName>().ok())
                    .collect::<Vec<_>>(),
            )
            .max_age(Duration::from_secs(value.max_age.unwrap_or_default()))
    }
}
