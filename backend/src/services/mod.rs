mod prelude {
    pub use crate::connector::ClientError;
    pub use crate::prelude::*;
    pub use axum::middleware::from_fn_with_state;
    pub use axum::{
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
        middleware::Next,
        Router,
    };
    pub use futures::stream::FuturesUnordered;
    pub use std::sync::Arc;
    pub use tokio::sync::Mutex;
    pub use tower_sessions::Session;
}

pub mod api;
pub mod auth;

use crate::root;
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
        .api_route("/root", &Method::GET, root)
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
