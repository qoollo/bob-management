#![allow(clippy::multiple_crate_versions)]

use axum::Router;
use bob_management::{
    config::ConfigExt,
    prelude::*,
    root,
    router::{ApiV1, ApiVersion, NoApi, RouterApiExt},
    services::api_router_v1,
    ApiDoc,
};
use cli::Parser;
use error_stack::{Result, ResultExt};
use hyper::Method;
use std::{env, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::Level;

const FRONTEND_FOLDER: &str = "frontend";

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
    #[cfg(all(feature = "swagger", debug_assertions))]
    let app = app.merge(bob_management::openapi_doc());

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

#[allow(clippy::unwrap_used, clippy::expect_used)]
fn router(cors: CorsLayer) -> Router {
    let mut frontend = env::current_exe().expect("Couldn't get current executable path.");
    frontend.pop();
    frontend.push(FRONTEND_FOLDER);
    tracing::info!("serving frontend at: {frontend:?}");
    let router = Router::new()
        // Frontend
        .nest_service("/", ServeDir::new(frontend));

    // Add API
    let router = router
        .with_context::<NoApi, ApiDoc>()
        .api_route("/root", &Method::GET, root)
        .unwrap()
        .expect("Couldn't register new API route");

    router
        .nest(
            ApiV1::to_path(),
            api_router_v1().expect("couldn't get API routes"),
        )
        .layer(ServiceBuilder::new().layer(cors))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use bob_management::services::api_router_v1;

    #[test]
    fn register_routes() {
        let _ = api_router_v1().expect("Router has invalid API methods");
    }
}
