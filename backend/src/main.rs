#![allow(clippy::multiple_crate_versions)]

use axum::{routing::get, Router};
use backend::{prelude::*, root, services::api_router};
use cli::Parser;
use error_stack::Report;
use std::path::PathBuf;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::Level;

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() -> Result<(), InitServerError> {
    let config: cli::Config = cli::Args::parse().try_into().map_err(|e| {
        Report::new(InitServerError).attach_printable(format!("couldn't get config file: {e}"))
    })?;

    let logger: cli::LoggerConfig = cli::Args::parse().try_into().map_err(|e| {
        Report::new(InitServerError)
            .attach_printable(format!("couldn't get logger configuration file: {e}"))
    })?;

    init_tracer(&logger.log_file, logger.trace_level);
    tracing::info!("Logger: {logger:?}");

    let cors: CorsLayer = config.cors_allow_all.clone().into();
    tracing::info!("CORS: {cors:?}");

    let addr = config.address;
    tracing::info!("listening on {addr}");

    let app = router(cors);
    #[cfg(feature = "swagger")]
    let app = app.merge(backend::openapi_doc());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            Report::new(InitServerError)
                .attach_printable(format!("failed to start axum server: {e}"))
        })?;

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
