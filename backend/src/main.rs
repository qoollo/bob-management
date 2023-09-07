#![allow(clippy::multiple_crate_versions)]
use std::path::PathBuf;

use axum::{routing::get, Router};
use backend::{prelude::*, root, services::api_router};
use cli::Parser;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::Level;

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config: cli::Config = cli::Args::parse()
        .try_into()
        .expect("couldn't get config file");

    let logger: cli::Logger = cli::Args::parse()
        .try_into()
        .expect("couldn't get logger configuration file");
    init_tracer(&logger.log_file, logger.trace_level);
    tracing::info!("Logger: {logger:?}");

    let cors: CorsLayer = config.cors.clone().into();
    tracing::info!("CORS: {cors:?}");

    let addr = config.address;
    tracing::info!("listening on {addr}");

    let app = router(cors);
    #[cfg(feature = "swagger")]
    let app = app.merge(backend::openapi_doc());

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
