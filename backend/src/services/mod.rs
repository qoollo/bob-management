use axum::{
    response::{IntoResponse, Response},
    Router,
};
use hyper::{Body, StatusCode};

/// Export all secured routes
#[allow(dead_code)]
pub fn api_router() -> Router<(), Body> {
    Router::new()
}

/// Errors that happend during API request proccessing
#[derive(Debug)]
pub enum APIError {
    RequestFailed,
    InvalidStatusCode(StatusCode),
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestFailed => f.write_str("The request to the specified resource failed"),
            Self::InvalidStatusCode(code) => f.write_fmt(format_args!(
                "Server received invalid status code from client: {code}"
            )),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        match self {
            Self::RequestFailed => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::InvalidStatusCode(code) => code.into_response(),
        }
    }
}
