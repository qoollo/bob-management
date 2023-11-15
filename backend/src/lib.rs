#![allow(
    async_fn_in_trait,
    clippy::multiple_crate_versions,
    clippy::module_name_repetitions
)]

#[cfg(all(feature = "swagger", debug_assertions))]
use axum::{routing::get, Router};

#[allow(unused_imports)]
use prelude::*;

pub mod config;
pub mod connector;
pub mod error;
pub mod models;
pub mod router;
pub mod services;

struct SecurityAddon;

#[cfg(all(feature = "swagger", debug_assertions))]
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("bob_apikey"))),
            );
        }
    }
}

#[cfg_attr(all(feature = "swagger", debug_assertions), derive(OpenApi))]
#[cfg_attr(all(feature = "swagger", debug_assertions), openapi(
    paths(
        root,
        services::auth::login,
        services::auth::logout,
        services::api::get_disks_count,
        services::api::get_nodes_count,
        services::api::get_rps,
        services::api::get_space,
    ),
    components(
        schemas(models::shared::Credentials, models::shared::Hostname, models::shared::BobConnectionData,
            models::api::DiskProblem,
            models::api::DiskStatus,
            models::api::DiskStatusName,
            models::api::DiskCount,
            models::api::NodeProblem,
            models::api::NodeStatus,
            models::api::NodeStatusName,
            models::api::NodeCount,
            models::api::ReplicaProblem,
            models::api::ReplicaStatus,
            models::api::SpaceInfo,
            models::api::VDiskStatus,
            models::api::Operation,
            models::api::RPS,
            models::api::RawMetricEntry,
            models::api::TypedMetrics,
            connector::dto::MetricsEntryModel,
            connector::dto::MetricsSnapshotModel,
            connector::dto::NodeConfiguration
        )
    ),
    tags(
        (name = "bob", description = "BOB management API")
    ),
    modifiers(&SecurityAddon)
))]
pub struct ApiDoc;

// [TEMP]
// TODO: Remove when the actual API will be implemented
#[allow(clippy::unused_async)]
#[cfg_attr(all(feature = "swagger", debug_assertions), utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/root",
        responses(
            (status = 200, description = "Hello Bob!")
        )
    ))]
pub async fn root() -> &'static str {
    "Hello Bob!"
}
/// Generate openapi documentation for the project
///
/// # Panics
///
/// Panics if `OpenAPI` couldn't be converted into YAML format
#[cfg(all(feature = "swagger", debug_assertions))]
#[allow(clippy::expect_used)]
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
    pub use crate::{
        connector::{
            client::Client,
            context::{ClientContext, ContextWrapper, DropContextService},
            dto, BobClient,
        },
        error::AppError,
        models::{
            bob::NodeName,
            shared::{BobConnectionData, Hostname, RequestTimeout, XSpanIdString},
        },
        router::{ApiV1, ApiVersion, RouteError, RouterApiExt},
        services::auth::HttpBobClient,
        ApiDoc,
    };
    pub use axum::{
        async_trait,
        headers::authorization::Basic,
        response::{IntoResponse, Response, Result as AxumResult},
        Extension, Json,
    };
    pub use error_stack::{Context, Report, Result, ResultExt};
    pub use hyper::{client::HttpConnector, Body, Method, Request, StatusCode};
    pub use serde::{Deserialize, Serialize};
    pub use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
        marker::PhantomData,
        str::FromStr,
        sync::Arc,
    };
    pub use thiserror::Error;
    #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::{
        openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
        IntoParams, Modify, OpenApi, PartialSchema, ToSchema,
    };
    pub use uuid::Uuid;
}

pub mod main {
    pub mod prelude {
        pub use crate::{
            config::{ConfigExt, LoggerExt},
            models::shared::RequestTimeout,
            prelude::*,
            root,
            router::{ApiV1, ApiVersion, NoApi, RouterApiExt},
            services::{
                api_router_v1,
                auth::{require_auth, AuthState, BobUser, HttpBobClient, InMemorySessionStore},
            },
            ApiDoc,
        };
        pub use axum::{
            error_handling::HandleErrorLayer, middleware::from_fn_with_state, BoxError, Extension,
            Router,
        };
        pub use cli::Parser;
        pub use error_stack::{Result, ResultExt};
        pub use hyper::{Method, StatusCode};
        pub use std::{env, path::PathBuf};
        pub use tower::ServiceBuilder;
        pub use tower_http::{cors::CorsLayer, services::ServeDir};
        pub use tower_sessions::{MemoryStore, SessionManagerLayer};
        pub use tracing::Level;
        pub use uuid::Uuid;
    }
}
