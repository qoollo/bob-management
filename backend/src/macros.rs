// NOTE: might be better to declare API as global constant, but the downside of this is
// that we need to pass around this variable alongside with macro call
// lazy_static! {
//     pub static ref API: OpenApi = ApiDoc::openapi();
// }

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("No route found in OpenAPI scheme")]
    NoRoute,
    #[error("No method found for specified route in OpenAPI scheme")]
    NoMethod,
    #[error("No `operation_id` found")]
    NoOperation,
    #[error("OpenAPI's `operation_id` doesn't match handler's name")]
    NoMatch,
}

// TODO: add all axum's routing options (wildcards, any, on, service routing) support
/// Check if the following route corresponds with `OpenAPI` declaration
/// Relies on `operation_id` field, should NOT be changed on new route declaration
#[macro_export]
macro_rules! new_api_route {
    ($router:expr, $route:literal, get($func:ident)) => {{
        new_api_route!($route, $func, Get).map(|_| $router.route($route, get($func)))
    }};
    ($router:expr, $route:literal, post($func:ident)) => {{
        new_api_route!($route, $func, Post).map(|_| $router.route($route, post($func)))
    }};
    ($router:expr, $route:literal, delete($func:ident)) => {{
        new_api_route!($route, $func, Delete).map(|_| $router.route($route, delete($func)))
    }};
    ($router:expr, $route:literal, put($func:ident)) => {{
        new_api_route!($route, $func, Put).map(|_| $router.route($route, put($func)))
    }};
    ($router:expr, $route:literal, head($func:ident)) => {{
        new_api_route!($route, $func, Head).map(|_| $router.route($route, head($func)))
    }};
    ($router:expr, $route:literal, options($func:ident)) => {{
        new_api_route!($route, $func, Options).map(|_| $router.route($route, options($func)))
    }};
    ($router:expr, $route:literal, patch($func:ident)) => {{
        new_api_route!($route, $func, Patch).map(|_| $router.route($route, patch($func)))
    }};
    ($router:expr, $route:literal, trace($func:ident)) => {{
        new_api_route!($route, $func, Trace).map(|_| $router.route($route, trace($func)))
    }};
    ($router:expr, $route:literal, connect($func:ident)) => {{
        new_api_route!($route, $func, Connect).map(|_| $router.route($route, connect($func)))
    }};
    ($route:literal, $func:ident, $method:ident) => {{
        || -> Result<(), RouteError> {
            Ok(ApiDoc::openapi()
                .paths
                .get_path_item(
                    $route
                        .split('/')
                        .map(|arg| {
                            arg.starts_with(':')
                                .then(|| ["{", &arg[1..], "}"].concat())
                                .unwrap_or_else(|| arg.to_string())
                        })
                        .collect::<Vec<_>>()
                        .join("/"),
                )
                .ok_or(RouteError::NoRoute)?
                .operations
                .get(&utoipa::openapi::PathItemType::$method)
                .ok_or(RouteError::NoMethod)?
                .operation_id
                .clone()
                .ok_or(RouteError::NoOperation)?
                .eq(&stringify!($func))
                .then_some(())
                .ok_or(RouteError::NoMatch)?)
        }
    }
    ()};
}
