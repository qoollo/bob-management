#![allow(clippy::module_name_repetitions)]
use std::fmt;

use axum::response::{IntoResponse, Response};
use error_stack::Context;
use hyper::StatusCode;
use thiserror::Error;

/// Generic error that wraps `error_stack::Context`.
/// Generally used for notifying the client that some error occurred on the server
///
/// For the internal server errors
#[derive(Debug)]
pub struct InternalAppError;

impl fmt::Display for InternalAppError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Interval Server Error")
    }
}

impl Context for InternalAppError {}

impl IntoResponse for InternalAppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct InitServerError;

impl fmt::Display for InitServerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("server initialization failed")
    }
}

impl Context for InitServerError {}

/// Errors that happend during API request proccessing
///
/// For errors that should be known on the client
#[derive(Error, Debug)]
pub enum APIError {
    #[error("the request to the specified resource failed")]
    RequestFailed,
    #[error("server received invalid status code from client")]
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
