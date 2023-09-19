#![allow(clippy::module_name_repetitions)]
use std::fmt;

use axum::response::{IntoResponse, Response};
use error_stack::Context;
use hyper::StatusCode;

/// Server start up errors
#[derive(Debug)]
pub enum AppError {
    InitializationError,
    StartUpError,
}

impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(match self {
            Self::InitializationError => "Server initialization failed",
            Self::StartUpError => "Server start up failed",
        })
    }
}

impl Context for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}
