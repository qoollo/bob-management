use crate::prelude::*;
use axum::body::HttpBody;
use axum::routing::on;
use axum::{handler::Handler, routing::MethodFilter, Router};
use hyper::{Body, Method};
use std::convert::Infallible;
use std::ops::Deref;
use thiserror::Error;
use utoipa::openapi::PathItemType;
use utoipa::OpenApi;

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
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

pub trait RouterApiExt<S = (), B = Body, E = Infallible> {
    ///  Add API Route
    ///   
    fn api_route<'a, H, T, Version, ApiDocumentation>(
        self,
        path: &str,
        method: Method,
        handler: H,
    ) -> Result<Self, RouteError>
    where
        H: Handler<T, S, B>,
        T: 'static,
        S: Send + Sync + 'static,
        Version: ApiVersion<'a>,
        ApiDocumentation: OpenApi,
        Self: Sized;
}

impl<S, B> RouterApiExt<S, B, Infallible> for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn api_route<'a, H, T, Version, ApiDocumentation>(
        self,
        path: &str,
        method: Method,
        handler: H,
    ) -> Result<Self, RouteError>
    where
        H: Handler<T, S, B>,
        T: 'static,
        S: Send + Sync + 'static,
        Version: ApiVersion<'a>,
        ApiDocumentation: OpenApi,
        Self: Sized,
    {
        check_api::<_, _, _, H, Version, ApiDocumentation>(
            path,
            &*TryInto::<MethodWrapper<PathItemType>>::try_into(method.clone())?,
        )?;
        Ok(self.route(
            path,
            on(
                *TryInto::<MethodWrapper<MethodFilter>>::try_into(method)?,
                handler,
            ),
        ))
    }
}

/// Check if the following route corresponds with `OpenAPI` declaration
/// Relies on `operation_id` field, must NOT be changed on handler's declaration
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
    #[cfg(any(not(debug_assertions), not(feature = "swagger")))]
    return Ok(());
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

// TODO: Restrict input types by some trait?
pub struct MethodWrapper<T>(T);

impl Deref for MethodWrapper<PathItemType> {
    type Target = PathItemType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for MethodWrapper<MethodFilter> {
    type Target = MethodFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// FIXME: Is there a better way to do this?..
impl TryFrom<Method> for MethodWrapper<PathItemType> {
    type Error = RouteError;

    fn try_from(value: Method) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            Method::GET => Self(PathItemType::Get),
            Method::PUT => Self(PathItemType::Put),
            Method::POST => Self(PathItemType::Post),
            Method::HEAD => Self(PathItemType::Head),
            Method::PATCH => Self(PathItemType::Patch),
            Method::TRACE => Self(PathItemType::Trace),
            Method::DELETE => Self(PathItemType::Delete),
            Method::OPTIONS => Self(PathItemType::Options),
            Method::CONNECT => Self(PathItemType::Connect),
            _ => Err(RouteError::UnexpectedMethod)?,
        })
    }
}

impl TryFrom<Method> for MethodWrapper<MethodFilter> {
    type Error = RouteError;

    fn try_from(value: Method) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            Method::GET => Self(MethodFilter::GET),
            Method::PUT => Self(MethodFilter::PUT),
            Method::POST => Self(MethodFilter::POST),
            Method::HEAD => Self(MethodFilter::HEAD),
            Method::PATCH => Self(MethodFilter::PATCH),
            Method::TRACE => Self(MethodFilter::TRACE),
            Method::DELETE => Self(MethodFilter::DELETE),
            Method::OPTIONS => Self(MethodFilter::OPTIONS),
            Method::CONNECT => Err(RouteError::UnexpectedMethod)?,
            _ => Err(RouteError::UnexpectedMethod)?,
        })
    }
}

mod tests {
    #![allow(dead_code, clippy::unused_async, clippy::unwrap_used)]
    use super::*;

    #[derive(OpenApi)]
    #[openapi(paths(test_route, test_root_route, connect_route))]
    pub struct TestDoc;
    #[utoipa::path(get, path = "/test")]
    async fn test_route() {}
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
    fn correct_api() {
        assert!(Router::<(), Body>::new()
            .api_route::<_, _, NoApi, TestDoc>("/test", Method::GET, test_route)
            .is_ok());
    }

    #[test]
    fn incorrect_path() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/tester", Method::GET, test_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }

    #[test]
    fn incorrect_method() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/test", Method::POST, test_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoMethod
        );
    }

    #[test]
    fn mismatched_path() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/context/", Method::GET, test_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoMatch
        );
    }

    #[test]
    fn unexpected_method_connect() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/connect", Method::CONNECT, connect_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::UnexpectedMethod
        );
    }

    #[test]
    fn correct_context_raw() {
        assert!(Router::<(), Body>::new()
            .api_route::<_, _, NoApi, TestDoc>("/context/", Method::GET, test_root_route)
            .is_ok());
    }

    #[test]
    fn correct_context_version() {
        assert!(Router::<(), Body>::new()
            .api_route::<_, _, ApiContext, TestDoc>("/", Method::GET, test_root_route)
            .is_ok());
    }

    #[test]
    fn no_context() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/", Method::GET, test_root_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }

    #[test]
    fn incorrect_context() {
        assert_eq!(
            Router::<(), Body>::new()
                .api_route::<_, _, NoApi, TestDoc>("/contexting/", Method::GET, test_root_route)
                .err()
                .unwrap()
                .current_context(),
            &RouteError::NoRoute
        );
    }
}
