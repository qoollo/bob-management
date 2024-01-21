use super::prelude::*;
use std::result::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub struct Hostname(
    #[serde(
        deserialize_with = "hyper_serde::deserialize",
        serialize_with = "hyper_serde::serialize"
    )]
    Uri,
);

#[derive(Debug, Error)]
pub enum HostnameError {
    #[error("bad address: no port")]
    NoPort,
    #[error("bad address: couldn't parse hostname")]
    BadAddress,
}

impl Hostname {
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.0.port_u16()
    }

    /// Creates [`Hostname`] from string with specified port
    ///
    /// # Errors
    ///
    /// This function will return an error if address doesn't have a port or a port is invalid
    pub fn with_port(address: &str, port: u16) -> error_stack::Result<Self, HostnameError> {
        let (body, _) = address.rsplit_once(':').ok_or(HostnameError::NoPort)?;
        let mut body = body.to_string();
        body.push_str(&format!(":{port}"));

        Ok(Self(
            hyper::http::Uri::from_str(&body).change_context(HostnameError::BadAddress)?,
        ))
    }
}

impl TryFrom<SocketAddr> for Hostname {
    type Error = <Uri as TryFrom<String>>::Error;

    fn try_from(value: SocketAddr) -> Result<Self, Self::Error> {
        Ok(Self(Uri::try_from(value.to_string())?))
    }
}

impl TryFrom<Hostname> for SocketAddr {
    type Error = std::net::AddrParseError;

    fn try_from(value: Hostname) -> Result<Self, Self::Error> {
        value.to_string().parse()
    }
}

impl ToString for Hostname {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// Data needed to connect to a BOB cluster
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    all(feature = "swagger", debug_assertions),
    derive(IntoParams, ToSchema)
)]
#[cfg_attr(all(feature = "swagger", debug_assertions),
    schema(example = json!({"hostname": "0.0.0.0:7000", "credentials": {"login": "archeoss", "password": "12345"}})))]
pub struct BobConnectionData {
    /// Address to connect to
    pub hostname: Hostname,

    /// [Optional] Credentials used for BOB authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

/// Optional auth credentials for a BOB cluster
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
#[cfg_attr(all(feature = "swagger", debug_assertions), schema(example = json!({"login": "archeoss", "password": "12345"})))]
pub struct Credentials {
    /// Login used during auth
    pub login: String,

    /// Password used during auth
    pub password: String,
}

#[allow(clippy::missing_fields_in_debug)]
impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("login", &self.login)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequestTimeout(Duration);

impl RequestTimeout {
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }

    #[must_use]
    pub const fn into_inner(self) -> Duration {
        self.0
    }
}

impl From<Duration> for RequestTimeout {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

/// Header - `X-Span-ID` - used to track a request through a chain of microservices.
pub const X_SPAN_ID: &str = "X-Span-ID";

/// Wrapper for a string being used as an X-Span-ID.
#[derive(Debug, Clone)]
pub struct XSpanIdString(pub String);

impl XSpanIdString {
    /// Extract an X-Span-ID from a request header if present, and if not
    /// generate a new one.
    pub fn get_or_generate<T>(req: &hyper::Request<T>) -> Self {
        let x_span_id = req.headers().get(X_SPAN_ID);

        x_span_id
            .and_then(|x| x.to_str().ok())
            .map(|x| Self(x.to_string()))
            .unwrap_or_else(Self::gen)
    }

    /// Generate Random `X-Span-ID` string.
    pub fn gen() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
