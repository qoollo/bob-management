//!
//! This file was *partly* auto-generated using OpenAPI server generator
//!     at <https://openapi-generator.tech/docs/generators/rust>
//!     on 2023-07-07
//!     using BOB's REST API schema, commit = ade0eadf1db7cda072cfab07dff7b1b57247e34a:
//!     <https://github.com/qoollo/bob/blob/928faef96ced755b75e3396b84febad1ecaf1dae/config-examples/openapi.yaml>
//!
//! This file was modified in order to get rid of the "swagger" crate, which brings
//!     a lot of unnecessary dependencies (for example, openssl, which can cause problems
//!     when creating docker images, even if we use only the http client). In addition,
//!     some refactoring was done in order to:
//!         1. Reduce the code size (from 2k-ish LOC to 500).
//!         2. Provide new functionality, e.g. adding authorization (and other) headers for
//!            new requests
//!         3. Unify error handling with `error_stack`
//!

#![allow(
    missing_docs,
    clippy::module_name_repetitions,
    dead_code,
    unused_variables
)]

use hyper::body::to_bytes;

use super::{api::prelude::*, prelude::*};

/// Error type for failing to create a Client
#[derive(Debug, Error)]
pub enum ClientInitError {
    #[error("invlaid URL scheme")]
    InvalidScheme,

    #[error("invlaid URI scheme")]
    InvalidUri,

    #[error("no hostname specified")]
    MissingHost,
}

/// A client that implements the API by making HTTP calls out to a server.
#[derive(Clone)]
pub struct Client<S, C, Cr>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<super::api::ServiceError> + std::fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Context Marker
    con_marker: PhantomData<fn(C)>,

    /// Credentials Marker
    cred_marker: PhantomData<fn(Cr)>,
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<std::result::Result<(), Self::Error>> {
        match self {
            Self::Http(client) => client.poll_ready(cx),
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match self {
            Self::Http(client) => client.call(req),
        }
    }
}

impl<C, Cr> Client<DropContextService<HyperClient, C>, C, Cr>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. <http://0.0.0.0:8000>
    ///
    /// # Errors
    ///
    /// This function will return an error if base path isn't valid URL
    pub fn try_new(base_path: &str) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path).change_context(ClientInitError::InvalidUri)?;

        let scheme = uri.scheme_str().unwrap_or_else(|| {
            tracing::info!("couldn't locate URI scheme... Fallback to http");
            "http"
        });
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => HyperClient::Http(hyper::client::Client::builder().build(connector.build())),

            _ => {
                return Err(ClientInitError::InvalidScheme.into());
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            con_marker: PhantomData,
            cred_marker: PhantomData,
        })
    }
}

/// Convert input into a base path, e.g. <http://example:123>. Also checks the scheme as it goes.
fn into_base_path(
    input: impl TryInto<Uri, Error = hyper::http::uri::InvalidUri>,
    _correct_scheme: Option<&'static str>,
) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input
        .try_into()
        .change_context(ClientInitError::InvalidUri)?;

    let scheme = uri.scheme_str().unwrap_or_else(|| {
        tracing::info!("couldn't locate URI scheme... Fallback to http");
        "http"
    });

    // Check the scheme if necessary
    // if let Some(correct_scheme) = correct_scheme {
    //     if scheme != correct_scheme {
    //         return Err(ClientInitError::InvalidScheme);
    //     }
    // }

    let host = uri.host().ok_or(ClientInitError::MissingHost)?;
    let port = uri.port_u16().map(|x| format!(":{x}")).unwrap_or_default();
    Ok(format!(
        "{}://{}{}{}",
        scheme,
        host,
        port,
        uri.path().trim_end_matches('/')
    ))
}

impl<C, Cr>
    Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C, Cr>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. <http://www.example.com>
    ///
    /// # Errors
    ///
    /// This function will return an error if base path isn't valid URL
    pub fn try_new_http(base_path: &str) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

impl<Connector, C, Cr> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C, Cr>
where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of [`hyper::client::Connect`].
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. <http://www.my-api-implementation.com>
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    ///
    /// # Errors
    ///
    ///  The function will fail if base path isn't a valid URL
    ///
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError> {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            con_marker: PhantomData,
            cred_marker: PhantomData,
        })
    }
}

impl<S, C, Cr> Client<S, C, Cr>
where
    Cr: Credentials + Clone,
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<super::api::ServiceError> + std::fmt::Display + error_stack::Context,
    C: Clone + Send + Sync + Has<XSpanIdString> + Has<Option<Authorization<Cr>>>,
{
    fn form_request(
        &self,
        endpoint: &str,
        method: Method,
        context: &C,
    ) -> Result<Request<Body>, ClientError> {
        let uri = format!("{}{endpoint}", self.base_path);

        let uri = Uri::from_str(&uri).change_context(ClientError::BadUri)?;
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .change_context(ClientError::CantFormRequest)?;
        let xspan = Has::<XSpanIdString>::get(context);
        let header =
            HeaderValue::from_str(&xspan.0).change_context(ClientError::CantFormRequest)?;

        request
            .headers_mut()
            .insert(HeaderName::from_static("x-span-id"), header);

        let auth_data = Has::<Option<Authorization<Cr>>>::get(context);
        if let Some(auth) = auth_data {
            request
                .headers_mut()
                .typed_insert::<Authorization<Cr>>(auth.clone());
        }

        Ok(request)
    }

    async fn handle_response_json<R: for<'a> Deserialize<'a>, T>(
        &self,
        response: Response<Body>,
        body_handler: impl Fn(R) -> T + Send,
    ) -> Result<T, APIError> {
        let body = response.into_body();
        let body = to_bytes(body)
            .await
            .change_context(APIError::ResponseError)?;
        let body = std::str::from_utf8(&body).change_context(APIError::ResponseError)?;

        let body = serde_json::from_str::<R>(body)
            .change_context(APIError::ResponseError)
            .attach_printable("Response body did not match the schema")?;

        Ok(body_handler(body))
    }
}

impl<S, C, Cr> Client<S, C, Cr>
where
    Cr: Credentials + Clone,
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<super::api::ServiceError> + std::fmt::Display + error_stack::Context,
    C: Clone + Send + Sync + Has<RequestTimeout>,
{
    async fn call(&self, req: Request<Body>, cx: &C) -> Result<Response<Body>, APIError> {
        let timeout = Has::<RequestTimeout>::get(cx);
        tokio::time::timeout(
            timeout.clone().into_inner(),
            self.client_service.clone().call((req, cx.clone())),
        )
        .await
        .change_context(APIError::RequestFailed)
        .attach_printable("No Response received")?
        .change_context(APIError::RequestFailed)
        .attach_printable("Hyper error")
    }
}

impl<S, C, Cr> Api<C> for Client<S, C, Cr>
where
    Cr: Credentials + Clone,
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<super::api::ServiceError> + std::fmt::Display + error_stack::Context,
    C: Clone
        + Send
        + Sync
        + Has<XSpanIdString>
        + Has<RequestTimeout>
        + Has<Option<Authorization<Cr>>>,
{
    /// Return directory of alien
    #[must_use]
    async fn get_alien_dir(&self, context: &C) -> Result<GetAlienDirResponse, APIError> {
        let request = self
            .form_request("/alien/dir", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Dir| GetAlienDirResponse::Directory(body))
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetAlienDirResponse::PermissionDenied(body)
                })
                .await?),
            406 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetAlienDirResponse::NotAcceptableBackend(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns the list of disks with their states
    #[must_use]
    async fn get_disks(&self, context: &C) -> Result<GetDisksResponse, APIError> {
        let request = self
            .form_request("/disks/list", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Vec<DiskState>| {
                    GetDisksResponse::AJSONArrayWithDisksAndTheirStates(body)
                })
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetDisksResponse::PermissionDenied(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Get metrics
    #[must_use]
    async fn get_metrics(&self, context: &C) -> Result<GetMetricsResponse, APIError> {
        let request = self
            .form_request("/metrics", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: MetricsSnapshotModel| {
                    GetMetricsResponse::Metrics(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns a list of known nodes
    #[must_use]
    async fn get_nodes(&self, context: &C) -> Result<GetNodesResponse, APIError> {
        let request = self
            .form_request("/nodes", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        // let update_context: RefClient = ref_self;
        // let response = update_context.call(request, context).await?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Vec<Node>| {
                    GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(body)
                })
                .await?),
            403 => Ok(GetNodesResponse::PermissionDenied),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns a partition info by ID
    #[must_use]
    async fn get_partition(
        &self,
        param_v_disk_id: i32,
        param_partition_id: String,
        context: &C,
    ) -> Result<GetPartitionResponse, APIError> {
        let request = self
            .form_request(
                &format!("/vdisks/{param_v_disk_id}/partitions/{param_partition_id}"),
                Method::GET,
                context,
            )
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Partition| {
                    GetPartitionResponse::AJSONWithPartitionInfo(body)
                })
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetPartitionResponse::PermissionDenied(body)
                })
                .await?),
            404 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetPartitionResponse::NotFound(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns a list of partitions
    #[must_use]
    async fn get_partitions(
        &self,
        param_v_disk_id: i32,
        context: &C,
    ) -> Result<GetPartitionsResponse, APIError> {
        let request = self
            .form_request(
                &format!("/vdisks/{param_v_disk_id}/partitions"),
                Method::GET,
                context,
            )
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: VDiskPartitions| {
                    GetPartitionsResponse::NodeInfoAndJSONArrayWithPartitionsInfo(body)
                })
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetPartitionsResponse::PermissionDenied(body)
                })
                .await?),

            404 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetPartitionsResponse::NotFound(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns count of records of this on node
    #[must_use]
    async fn get_records(
        &self,
        param_v_disk_id: i32,
        context: &C,
    ) -> Result<GetRecordsResponse, APIError> {
        let request = self
            .form_request(
                &format!("/vdisks/{param_v_disk_id}/records/count"),
                Method::GET,
                context,
            )
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, GetRecordsResponse::RecordsCount)
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetRecordsResponse::PermissionDenied(body)
                })
                .await?),
            404 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetRecordsResponse::NotFound(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns directories of local replicas of vdisk
    #[must_use]
    async fn get_replicas_local_dirs(
        &self,
        param_v_disk_id: i32,
        context: &C,
    ) -> Result<GetReplicasLocalDirsResponse, APIError> {
        let request = self
            .form_request(
                &format!("/vdisks/{param_v_disk_id}/replicas/local/dirs"),
                Method::GET,
                context,
            )
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Vec<Dir>| {
                    GetReplicasLocalDirsResponse::AJSONArrayWithDirs(body)
                })
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetReplicasLocalDirsResponse::PermissionDenied(body)
                })
                .await?),
            404 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetReplicasLocalDirsResponse::NotFound(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Get space info
    #[must_use]
    async fn get_space_info(&self, context: &C) -> Result<GetSpaceInfoResponse, APIError> {
        let request = self
            .form_request("/status/space", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: SpaceInfo| {
                    GetSpaceInfoResponse::SpaceInfo(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns information about self
    #[must_use]
    async fn get_status(&self, context: &C) -> Result<GetStatusResponse, APIError> {
        let request = self
            .form_request("/status", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Node| {
                    GetStatusResponse::AJSONWithNodeInfo(body)
                })
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns a vdisk info by ID
    #[must_use]
    async fn get_v_disk(
        &self,
        param_v_disk_id: i32,
        context: &C,
    ) -> Result<GetVDiskResponse, APIError> {
        let request = self
            .form_request(&format!("/vdisks/{param_v_disk_id}"), Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: VDisk| {
                    GetVDiskResponse::AJSONWithVdiskInfo(body)
                })
                .await?),
            403 => Ok(self
                .handle_response_json(response, |body: StatusExt| {
                    GetVDiskResponse::PermissionDenied(body)
                })
                .await?),
            404 => Ok(self
                .handle_response_json(response, |body: StatusExt| GetVDiskResponse::NotFound(body))
                .await?),
            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns a list of vdisks
    #[must_use]
    async fn get_v_disks(&self, context: &C) -> Result<GetVDisksResponse, APIError> {
        let request = self
            .form_request("/vdisks", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: Vec<VDisk>| {
                    GetVDisksResponse::AJSONArrayOfVdisksInfo(body)
                })
                .await?),
            403 => Ok(GetVDisksResponse::PermissionDenied),

            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns server version
    #[must_use]
    async fn get_version(&self, context: &C) -> Result<GetVersionResponse, APIError> {
        let request = self
            .form_request("/version", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: VersionInfo| {
                    GetVersionResponse::VersionInfo(body)
                })
                .await?),

            _ => Err(APIError::from(response))?,
        }
    }

    /// Returns configuration of the node
    #[must_use]
    async fn get_configuration(&self, context: &C) -> Result<GetConfigurationResponse, APIError> {
        let request = self
            .form_request("/configuration", Method::GET, context)
            .change_context(APIError::RequestFailed)?;
        let response = self.call(request, context).await?;

        match response.status().as_u16() {
            200 => Ok(self
                .handle_response_json(response, |body: NodeConfiguration| {
                    GetConfigurationResponse::ConfigurationObject(body)
                })
                .await?),
            403 => Ok(GetConfigurationResponse::PermissionDenied),

            _ => Err(APIError::from(response))?,
        }
    }
}

impl From<Response<Body>> for APIError {
    fn from(response: Response<Body>) -> Self {
        let code = response.status();

        Self::InvalidStatusCode(code)
    }
}
