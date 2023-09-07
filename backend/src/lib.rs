#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

#[cfg(all(feature = "swagger", debug_assertions))]
use axum::{routing::get, Router};

use utoipa::OpenApi;
pub mod config;
pub mod connector;
pub mod error;
pub mod models;
pub mod router;
pub mod services;

#[derive(OpenApi)]
#[cfg_attr(not(all(feature = "swagger", debug_assertions)), openapi())]
#[cfg_attr(all(feature = "swagger", debug_assertions), openapi(
    paths(root),
    tags(
        (name = "bob", description = "BOB management API")
    )
))]
pub struct ApiDoc;

// [TEMP]
// TODO: Remove when the actual API will be implemented
#[allow(clippy::unused_async)]
#[cfg_attr(all(feature = "swagger", debug_assertions), utoipa::path(
        get,
        path = "/root",
        responses(
            (status = 200, description = "Hello Bob!")
        )
    ))]
pub async fn root() -> &'static str {
    "Hello Bob!"
}

/// Generate openapi documentation for the project
#[cfg(all(feature = "swagger", debug_assertions))]
pub fn openapi_doc() -> Router {
    use utoipa_rapidoc::RapiDoc;
    use utoipa_redoc::{Redoc, Servable};
    use utoipa_swagger_ui::SwaggerUi;

    /* Swagger-only routes */
    tracing::info!("Swagger ui available at /swagger-ui");

    /* Mount Swagger ui */
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route(
            "/api-docs/openapi.yaml",
            get(|| async {
                ApiDoc::openapi()
                    .to_yaml()
                    .expect("Couldn't produce .yaml API scheme")
            }),
        )
    // Alternative to above
    // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
}

pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::error::AppError;
    pub use crate::router::RouteError;
    pub use axum::response::Result as AxumResult;
    pub use error_stack::{Context, Report, Result, ResultExt};
    // #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::OpenApi;
}
