#![allow(clippy::multiple_crate_versions)]

use axum::{routing::get, Router};
use backend::{config::ConfigExt, prelude::*, root, services::api_router};
use cli::Parser;
use error_stack::{Result, ResultExt};
use std::{env, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::Level;

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() -> Result<(), AppError> {
    let config = cli::Config::try_from(cli::Args::parse())
        .change_context(AppError::InitializationError)
        .attach_printable("Couldn't get config file.")?;

    let logger = &config.logger;

    init_tracer(&logger.log_file, logger.trace_level);
    tracing::info!("Logger: {logger:?}");

    let cors: CorsLayer = config.get_cors_configuration();
    tracing::info!("CORS: {cors:?}");

    let addr = config.address;
    tracing::info!("Listening on {addr}");

    let app = router(cors);
    #[cfg(feature = "swagger")]
    let app = app.merge(backend::openapi_doc());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .change_context(AppError::StartUpError)
        .attach_printable("Failed to start axum server")?;

    Ok(())
}

fn init_tracer(_log_file: &Option<PathBuf>, trace_level: Level) {
    let subscriber = tracing_subscriber::fmt().with_max_level(trace_level);
    subscriber.init();
}

fn router(cors: CorsLayer) -> Router {
    let mut frontend = env::current_exe().expect("Couldn't get current executable path.");
    frontend.pop();
    frontend.push("dist");
    tracing::info!("serving frontend at: {frontend:?}");
    // Add api
    Router::new()
        // Frontend
        .nest_service("/", ServeDir::new(frontend))
        // Unsecured Routes
        .route("/root", get(root))
        .nest("/api", api_router())
        .layer(ServiceBuilder::new().layer(cors))
}
