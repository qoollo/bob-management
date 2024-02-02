mod prelude {
    pub use crate::{
        connector::{
            api::{prelude::*, ApiNoContext},
            ClientError,
        },
        models::api::*,
        prelude::*,
    };
    pub use axum::{
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
        middleware::{from_fn_with_state, Next},
        Router,
    };
    pub use futures::{stream::FuturesUnordered, StreamExt};
    pub use std::sync::Arc;
    pub use tokio::sync::Mutex;
    pub use tower_sessions::Session;
}

pub mod api;
pub mod auth;
pub mod methods;

use api::{get_disks_count, get_nodes_count, get_rps, get_space};
use auth::{login, logout, require_auth, AuthState, BobUser, HttpBobClient, InMemorySessionStore};
use prelude::*;

type BobAuthState = AuthState<
    BobUser,
    Uuid,
    InMemorySessionStore<Uuid, BobUser>,
    InMemorySessionStore<Uuid, HttpBobClient>,
>;

/// Export all secured API routes
///
/// # Errors
///
/// This function will return an error if one of the routes couldn't be registred
#[allow(dead_code)]
pub fn api_router_v1(auth_state: BobAuthState) -> Result<Router<BobAuthState>, RouteError> {
    Router::new()
        .with_context::<ApiV1, ApiDoc>()
        .api_route("/disks/count", &Method::GET, get_disks_count)
        .api_route("/nodes/count", &Method::GET, get_nodes_count)
        .api_route("/nodes/rps", &Method::GET, get_rps)
        .api_route("/nodes/space", &Method::GET, get_space)
        .unwrap()?
        .route_layer(from_fn_with_state(auth_state, require_auth))
        .with_context::<ApiV1, ApiDoc>()
        .api_route("/logout", &Method::POST, logout)
        .api_route("/login", &Method::POST, login)
        .unwrap()
}

/// Errors that happend during API request proccessing
#[derive(Debug, Error)]
pub enum APIError {
    #[error("The request to the specified resource failed")]
    RequestFailed,
    #[error("Server received invalid status code from client: `{0}`")]
    InvalidStatusCode(StatusCode),
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            Self::RequestFailed => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::InvalidStatusCode(code) => code.into_response(),
        }
    }
}
