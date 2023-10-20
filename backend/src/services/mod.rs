use crate::{
    prelude::*,
    router::{ApiV1, ContextRouter},
    ApiDoc,
};
use axum::{
    response::{IntoResponse, Response},
    Router,
};
use hyper::{Body, StatusCode};
use thiserror::Error;

/// Export all secured routes
#[allow(dead_code)]
pub fn api_router_v1() -> Result<Router<(), Body>, RouteError> {
    ContextRouter::<ApiV1, ApiDoc>::new().unwrap()
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
