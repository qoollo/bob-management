mod prelude {
    pub use super::api::prelude::*;
    pub use super::{
        context::{ContextWrapper, DropContextService, Has},
        ClientError, Connector,
    };
    pub use crate::{models::shared::XSpanIdString, prelude::*, services::auth::HttpClient};
    pub use axum::{
        headers::{authorization::Credentials, Authorization, HeaderMapExt},
        http::{HeaderName, HeaderValue},
    };
    pub use futures::StreamExt;
    pub use hyper::{service::Service, Response, Uri};
    pub use std::{
        str::FromStr,
        sync::Arc,
        task::{Context, Poll},
    };
}

use api::{ApiNoContext, ContextWrapperExt};
use client::Client;
use context::ClientContext;
use prelude::*;

use self::error::AsApiError;

pub mod api;
pub mod client;
pub mod context;
pub mod dto;
pub mod error;

pub type ApiInterface = dyn ApiNoContext<ClientContext> + Send + Sync;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("couldn't init http client")]
    InitClient,
    #[error("couldn't probe connection to the node")]
    Inaccessible,
    #[error("permission denied")]
    PermissionDenied,
    #[error("no client found for requested resource")]
    NoClient,
    #[error("can't form hyper request")]
    CantFormRequest,
    #[error("bad URI")]
    BadUri,
}

/// HTTP Connector constructor
#[derive(Debug)]
pub struct Connector;

impl Connector {
    /// Alows building a HTTP(S) connector. Used for instantiating clients with custom
    /// connectors.
    #[must_use]
    pub const fn builder() -> Builder {
        Builder {}
    }
}

/// Builder for HTTP(S) connectors
#[derive(Debug)]
pub struct Builder {}

impl Builder {
    /// [Stub] Use HTTPS instead of HTTP
    #[must_use]
    pub const fn https(self) -> HttpsBuilder {
        HttpsBuilder {}
    }

    /// Build a HTTP connector
    #[must_use]
    pub fn build(self) -> hyper::client::connect::HttpConnector {
        hyper::client::connect::HttpConnector::new()
    }
}

// TODO
/// [Stub] Builder for HTTPS connectors
#[derive(Debug)]
pub struct HttpsBuilder {}

impl HttpsBuilder {
    pub fn build(self) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct BobClient<Client: ApiNoContext<ClientContext> + Send + Sync> {
    /// Unique Identifier
    id: Uuid,

    /// Bob's hostname
    hostname: Hostname,

    // NOTE: Can (and should) the API interface mutate?..
    /// Connection,
    main: Arc<Client>,

    /// Clients for all known nodes
    cluster: HashMap<NodeName, Arc<Client>>,
}

#[allow(clippy::missing_fields_in_debug)]
impl<Client: ApiNoContext<ClientContext> + Send + Sync + Clone> std::fmt::Debug
    for BobClient<Client>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = &self
            .main
            .context()
            .auth_data
            .as_ref()
            .map_or("Unknown", |cred| cred.username());
        f.debug_struct("BobClient")
            .field("hostname", &self.hostname)
            .field("user", &user)
            .finish()
    }
}

impl<ApiInterface: ApiNoContext<ClientContext> + Send + Sync> BobClient<ApiInterface> {
    /// Creates new [`BobClient`] from [`BobConnectionData`]
    ///
    /// # Errors
    /// The function will fail if a hostname isn't a valid url or the client couldn't establish
    /// connection for the BOB cluster
    pub async fn try_new(
        bob_data: BobConnectionData,
        timeout: RequestTimeout,
    ) -> Result<HttpBobClient, ClientError> {
        let auth = bob_data
            .credentials
            .map(|creds| Authorization::basic(&creds.login, &creds.password));
        let hostname = bob_data.hostname.clone();

        let context: ClientContext = ClientContext {
            timeout,
            auth_data: auth,
            xspan: XSpanIdString::gen(),
        };
        let client = Client::try_new_http(&hostname.to_string())
            .change_context(ClientError::InitClient)
            .attach_printable(format!("Hostname: {}", hostname.to_string()))?;
        let nodes_resp = client
            .clone()
            .with_context(context.clone())
            .get_nodes()
            .await
            .change_context(ClientError::Inaccessible)
            .attach_printable(format!("Hostname: {}", hostname.to_string()))?;
        let api::GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(nodes) = nodes_resp else {
            Err(nodes_resp.as_invalid_status())
                .change_context(ClientError::Inaccessible)
                .attach_printable(format!("Hostname: {}", hostname.to_string()))?
        };

        let cluster: HashMap<NodeName, Arc<_>> = nodes
            .iter()
            .filter_map(|node| HttpClient::from_node(node, &bob_data.hostname, context.clone()))
            .collect();

        Ok(BobClient {
            id: Uuid::new_v4(),
            hostname: bob_data.hostname,
            main: Arc::new(client.with_context(context)),
            cluster,
        })
    }

    /// Probes connection to the Bob's main connected node
    ///
    /// Returns `StatusCode::OK` on success
    ///
    /// # Errors
    ///
    /// The function fails if there was an error during creation of request
    /// It shouldn't happen on normal behaviour
    ///
    pub async fn probe_main(&self) -> Result<StatusCode, ClientError> {
        match self
            .main
            .get_nodes()
            .await
            .change_context(ClientError::Inaccessible)?
        {
            api::GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(_) => Ok(StatusCode::OK),
            api::GetNodesResponse::PermissionDenied => Err(ClientError::PermissionDenied.into()),
        }
    }

    /// Probes connection to the Bob's main connected node
    ///
    /// Returns `StatusCode::OK` on success
    ///
    /// # Errors
    ///
    /// The function fails if there was an error during creation of request
    /// It shouldn't happen on normal behaviour
    ///
    pub async fn probe_secondary(&self, node_name: &NodeName) -> Result<StatusCode, ClientError> {
        match self
            .cluster
            .get(node_name)
            .ok_or(ClientError::NoClient)?
            .get_nodes()
            .await
            .change_context(ClientError::Inaccessible)?
        {
            api::GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(_) => Ok(StatusCode::OK),
            api::GetNodesResponse::PermissionDenied => Err(ClientError::PermissionDenied.into()),
        }
    }

    /// Probes connection to all Bob's connected nodes
    ///
    /// Returns `StatusCode::OK` on success
    ///
    /// # Errors
    ///
    /// The function fails if there was an error during creation of request
    /// It shouldn't happen on normal behaviour
    ///
    // pub async fn probe_cluster(&self) -> Vec<(NodeName, StatusCode)> {
    pub async fn probe_cluster(&self) -> Vec<(NodeName, StatusCode)> {
        let v: Vec<_> = futures::stream::iter(&self.cluster)
            .map(|(node_name, node)| async {
                (
                    node_name.clone(),
                    match node.get_nodes().await {
                        Ok(api::GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(_)) => {
                            StatusCode::OK
                        }
                        Ok(api::GetNodesResponse::PermissionDenied) => StatusCode::UNAUTHORIZED,
                        Err(_) => StatusCode::NOT_FOUND,
                    },
                )
            })
            .collect()
            .await;

        futures::future::join_all(v).await
    }

    #[must_use]
    pub fn context(&self) -> &ClientContext {
        self.main.context()
    }

    #[must_use]
    pub fn api_main(&self) -> &ApiInterface {
        self.main.as_ref()
    }

    pub fn cluster(&self) -> impl Iterator<Item = &Arc<ApiInterface>> {
        self.cluster.values()
    }

    pub fn api_secondary(&self, node_name: &NodeName) -> Option<&ApiInterface> {
        self.cluster.get(node_name).map(std::convert::AsRef::as_ref)
    }

    #[must_use]
    pub const fn cluster_with_addr(&self) -> &HashMap<NodeName, Arc<ApiInterface>> {
        &self.cluster
    }

    #[must_use]
    pub const fn hostname(&self) -> &Hostname {
        &self.hostname
    }

    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }
}

impl HttpClient {
    fn from_node(
        node: &dto::Node,
        hostname: &Hostname,
        context: ClientContext,
    ) -> Option<(String, Arc<Self>)> {
        let Some(port) = hostname.port() else {
            return None;
        };
        let name = &node.name;
        let client = Hostname::with_port(&node.address, port).map_or_else(
            |_| {
                tracing::warn!("couldn't change port for {name}. Client won't be created");
                None
            },
            |hostname| Some((name, Client::try_new_http(&hostname.to_string()))),
        );
        if let Some((name, Ok(client))) = client {
            Some((name.clone(), Arc::new(client.with_context(context))))
        } else {
            tracing::warn!("couldn't create client for {name}");
            None
        }
    }
}
