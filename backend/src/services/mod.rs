use axum::Router;
use hyper::Body;

/// Export all secured routes
#[allow(dead_code)]
pub fn api_router() -> Router<(), Body> {
    Router::new()
}
