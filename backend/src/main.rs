#![allow(clippy::multiple_crate_versions)]

use axum::{routing::get, Router};
use backend::{config::ConfigExt, prelude::*, root, services::api_router};
use cli::Parser;
use error_stack::{Report, Result, ResultExt};
use std::path::PathBuf;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::Level;

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() -> Result<(), InitServerError> {
    let config: cli::Config = cli::Config::try_from(cli::Args::parse())
        .attach_printable(format!("couldn't get config file."))
        .change_context(InitServerError)?;

    let logger = &config.logger;

    init_tracer(&logger.log_file, logger.trace_level);
    tracing::info!("Logger: {logger:?}");

    let cors: CorsLayer = config.get_cors_configuration();
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
