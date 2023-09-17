use axum::Router;
use hyper::Body;

/// Export all secured routes
#[allow(dead_code)]
pub fn api_router() -> Router<(), Body> {
    Router::new()
}

// Errors that happend during API request proccessing
//
// For errors that should be known on the client
// #[derive(Error, Debug)]
// pub enum APIError {
//     #[error("the request to the specified resource failed")]
//     RequestFailed,
//     #[error("server received invalid status code from client")]
//     InvalidStatusCode(StatusCode),
// }
//
// impl IntoResponse for APIError {
//     fn into_response(self) -> Response {
//         match self {
//             Self::RequestFailed => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
//             Self::InvalidStatusCode(code) => code.into_response(),
//         }
//     }
// }
