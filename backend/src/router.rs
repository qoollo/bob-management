use crate::prelude::*;
use axum::body::HttpBody;
use axum::routing::on;
use axum::{handler::Handler, routing::MethodFilter, Router};
use hyper::{Body, Method};
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Deref;
use thiserror::Error;
use utoipa::openapi::PathItemType;
use utoipa::OpenApi;

#[derive(Clone, Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum RouteError {
    #[error("No route found in OpenAPI scheme")]
    NoRoute,
    #[error("No method found for specified route in OpenAPI scheme")]
    NoMethod,
    #[error("No `operation_id` found")]
    NoOperation,
    #[error("OpenAPI's `operation_id` doesn't match handler's name")]
    NoMatch,
    #[error("Unexpected Hyper method - was it `Method::CONNECT`?")]
    UnexpectedMethod,
    #[error("Error occured during handler processing")]
    InvalidHandler,
}

pub struct NoApi;

impl<'a> ApiVersion<'a> for NoApi {}

pub struct ApiV1;

pub trait ApiVersion<'a> {
    #[must_use]
    fn to_path() -> &'a str {
        ""
    }
}

impl<'a> ApiVersion<'a> for ApiV1 {
    fn to_path() -> &'a str {
        "/api/v1"
    }
}

pub struct ContextRouter<Version, Doc, S = (), B = Body> {
    inner: Router<S, B>,
    context: PhantomData<(Version, Doc)>,
    api_errors: Option<Report<RouteError>>,
}

impl<'a, Version, Doc, S, B> ContextRouter<Version, Doc, S, B>
where
    Version: ApiVersion<'a>,
    Doc: OpenApi,
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Router::<S, B>::new(),
            context: PhantomData,
            api_errors: None,
        }
    }

    /// Returns `Router` instance with new registred routes
    ///
    /// # Errors
    ///
    /// This function will return an error if one of the previously provided
    /// path/handler/method combinations with `api_route` does not have a corresponding `OpenAPI` declaration.
    /// The returned error in the form of a `Report` will contain all errors during new API route registrartion.
    pub fn unwrap(self) -> Result<Router<S, B>, RouteError> {
        self.api_errors.map_or(Ok(self.inner), Err)
    }

    #[must_use]
    pub fn change_context<V, D>(self) -> ContextRouter<V, D, S, B> {
        ContextRouter {
            inner: self.inner,
            context: PhantomData,
            api_errors: self.api_errors,
        }
    }

    /// Add API Route to the `Router`
    ///
    #[must_use]
    pub fn api_route<H, T>(mut self, path: &str, method: &Method, handler: H) -> Self
    where
        H: Handler<T, S, B>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
        B: HttpBody + Send + 'static,
    {
        #[cfg(all(feature = "swagger", debug_assertions))]
        match try_convert_path_item_type_from_method(method)
            .map(|path_item_type| check_api::<_, _, _, H, Version, Doc>(path, &path_item_type))
        {
            Ok(Ok(())) => (),
            Ok(Err(err)) | Err(err) => {
                if let Some(errors) = &mut self.api_errors {
                    errors.extend_one(err);
                } else {
                    self.api_errors = Some(err);
                }
            }
        }

        match try_convert_method_filter_from_method(method) {
            Ok(method) => self.inner = self.inner.route(path, on(method, handler)),
            Err(err) => {
                if let Some(errors) = &mut self.api_errors {
                    errors.extend_one(err);
                } else {
                    self.api_errors = Some(err);
                }
            }
        };

        self
    }
}

impl<'a, Version, Doc, S, B> Default for ContextRouter<Version, Doc, S, B>
where
    Version: ApiVersion<'a>,
    Doc: OpenApi,
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V, D, S, B> Deref for ContextRouter<V, D, S, B> {
    type Target = Router<S, B>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait RouterApiExt<S = (), B = Body, E = Infallible> {
    /// Wraps `Router` with `ApiVersion` and `OpenApi` instances into the new context to call
    /// `api_route` with said context
    fn with_context<Version, Doc>(self) -> ContextRouter<Version, Doc, S, B>;
}

impl<S, B> RouterApiExt<S, B, Infallible> for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn with_context<Version, Doc>(self) -> ContextRouter<Version, Doc, S, B> {
        ContextRouter {
            inner: self,
            context: PhantomData,
            api_errors: None,
        }
    }
}

/// Check if the following route corresponds with `OpenAPI` declaration
/// Relies on `operation_id` field, must NOT be changed on handler's declaration
#[cfg(all(feature = "swagger", debug_assertions))]
fn check_api<'a, T, S, B, H, Version, ApiDocumentation>(
    path: &str,
    method: &PathItemType,
) -> Result<(), RouteError>
where
    H: Handler<T, S, B>,
    T: 'static,
    S: Send + Sync + 'static,
    ApiDocumentation: OpenApi,
    Version: ApiVersion<'a>,
{
    let route = [
        Version::to_path(),
        &path
            .split('/')
            .map(|arg| {
                arg.starts_with(':')
                    .then(|| ["{", &arg[1..], "}"].concat())
                    .unwrap_or_else(|| arg.to_string())
            })
            .collect::<Vec<_>>()
            .join("/"),
    ]
    .concat();
    let operation_id = ApiDocumentation::openapi()
        .paths
        .get_path_item(&route)
        .ok_or(RouteError::NoRoute)
        .attach_printable(format!("route: {route}"))?
        .operations
        .get(method)
        .ok_or(RouteError::NoMethod)?
        .operation_id
        .clone()
        .ok_or(RouteError::NoOperation)?;
    let handler_name = &[std::any::type_name::<H>()
        .rsplit_once(':')
        .ok_or(RouteError::InvalidHandler)?
        .1]
    .concat();

    operation_id
        .eq(handler_name)
        .then_some(())
        .ok_or(RouteError::NoMatch)
        .attach_printable(format!("left: {operation_id}, right: {handler_name}"))
}

fn try_convert_path_item_type_from_method(value: &Method) -> Result<PathItemType, RouteError> {
    Ok(match *value {
        Method::GET => PathItemType::Get,
        Method::PUT => PathItemType::Put,
        Method::POST => PathItemType::Post,
        Method::HEAD => PathItemType::Head,
        Method::PATCH => PathItemType::Patch,
        Method::TRACE => PathItemType::Trace,
        Method::DELETE => PathItemType::Delete,
        Method::OPTIONS => PathItemType::Options,
        Method::CONNECT => PathItemType::Connect,
        _ => Err(RouteError::UnexpectedMethod)?,
    })
}

fn try_convert_method_filter_from_method(value: &Method) -> Result<MethodFilter, RouteError> {
    Ok(match *value {
        Method::GET => MethodFilter::GET,
        Method::PUT => MethodFilter::PUT,
        Method::POST => MethodFilter::POST,
        Method::HEAD => MethodFilter::HEAD,
        Method::PATCH => MethodFilter::PATCH,
        Method::TRACE => MethodFilter::TRACE,
        Method::DELETE => MethodFilter::DELETE,
        Method::OPTIONS => MethodFilter::OPTIONS,
        Method::CONNECT => Err(RouteError::UnexpectedMethod)?,
        _ => Err(RouteError::UnexpectedMethod)?,
    })
}

#[cfg(test)]
mod tests {
    #![allow(dead_code, clippy::unused_async, clippy::unwrap_used)]
    use super::*;

    #[derive(OpenApi)]
    #[openapi(paths(test_route, test_root_route, connect_route, test_post_route))]
    pub struct TestDoc;
    #[utoipa::path(get, path = "/test")]
    async fn test_route() {}
    #[utoipa::path(post, path = "/test_post")]
    async fn test_post_route() {}
    #[utoipa::path(get, context_path = "/context", path = "/")]
    async fn test_root_route() {}
    #[utoipa::path(connect, path = "/connect")]
    async fn connect_route() {}

    struct ApiContext;
    impl<'a> ApiVersion<'a> for ApiContext {
        fn to_path() -> &'a str {
            "/context"
        }
    }

    #[test]
    fn correct_api_with_context_wrapper() {
        let router = Router::<(), Body>::new()
            .with_context::<NoApi, TestDoc>()
            .api_route("/test", &Method::GET, test_route)
            .unwrap();

        assert!(router.is_ok(), "Err: {:?}", router.err().unwrap());
    }

    #[test]
    fn incorrect_path_with_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<NoApi, TestDoc>()
                .api_route("/tester", &Method::GET, test_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }

    #[test]
    fn incorrect_method_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<NoApi, TestDoc>()
                .api_route("/test", &Method::POST, test_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoMethod
        );
    }

    #[test]
    fn mismatched_path_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<NoApi, TestDoc>()
                .api_route("/context/", &Method::GET, test_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoMatch
        );
    }

    #[test]
    fn unexpected_method_connect_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<NoApi, TestDoc>()
                .api_route("/connect", &Method::CONNECT, connect_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::UnexpectedMethod
        );
    }

    #[test]
    fn correct_context_raw_context_wrapper() {
        let router = Router::<(), Body>::new()
            .with_context::<NoApi, TestDoc>()
            .api_route("/context/", &Method::GET, test_root_route)
            .unwrap();

        assert!(router.is_ok(), "Err: {:?}", router.err().unwrap());
    }

    #[test]
    fn correct_context_version_context_wrapper() {
        let router = Router::<(), Body>::new()
            .with_context::<ApiContext, TestDoc>()
            .api_route("/", &Method::GET, test_root_route)
            .unwrap();

        assert!(router.is_ok(), "Err: {:?}", router.err().unwrap());
    }

    #[test]
    fn no_context_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<NoApi, TestDoc>()
                .api_route("/", &Method::GET, test_root_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }

    #[test]
    fn incorrect_context_context_wrapper() {
        assert_eq!(
            Router::<(), Body>::new()
                .with_context::<ApiContext, TestDoc>()
                .api_route("/contexting/", &Method::GET, test_root_route)
                .unwrap()
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }

    #[test]
    fn multiple_contexts_context_wrapper() {
        let router = Router::<(), Body>::new()
            .with_context::<ApiContext, TestDoc>()
            .api_route("/", &Method::GET, test_root_route)
            .change_context::<NoApi, TestDoc>()
            .api_route("/test", &Method::GET, test_route)
            .unwrap()
            .unwrap()
            .route("/connect", axum::routing::post(connect_route))
            .with_context::<NoApi, TestDoc>()
            .api_route("/test_post", &Method::POST, test_post_route)
            .unwrap();

        assert!(router.is_ok(), "Err: {:?}", router.err().unwrap());
    }
}
