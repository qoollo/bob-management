#![allow(clippy::multiple_crate_versions)]
use std::path::PathBuf;

use axum::{routing::get, Router};
use backend::{prelude::*, root, services::api_router};
use cli::Parser;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config: cli::Config = cli::Args::parse()
        .try_into()
        .expect("couldn't get config file");

    init_tracer(&config.log_file, config.trace_level);

    let cors: CorsLayer = config.cors.clone().into();
    tracing::info!("CORS: {cors:?}");

    let addr = config.address;
    tracing::info!("listening on {addr}");

    let app = router(cors);
    #[cfg(debug_assertions)]
    let app = {
        /* Development-only routes */
        #[derive(OpenApi)]
        #[openapi(
                paths(backend::root),
                tags(
                    (name = "bob", description = "BOB management API")
                )
            )]
        struct ApiDoc;
        /* Mount Swagger ui */
        app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
            // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
            // via SwaggerUi instead we only make rapidoc to point to the existing doc.
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Alternative to above
        // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
    };

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn init_tracer(_log_file: &Option<PathBuf>, trace_level: Level) {
    let subscriber = tracing_subscriber::fmt().with_max_level(trace_level);
    subscriber.init();
}

fn router(cors: CorsLayer) -> Router {
    // Add api
    Router::new()
        // Unsecured Routes
        .route("/", get(root))
        .nest("/api", api_router())
        .layer(ServiceBuilder::new().layer(cors))
}
