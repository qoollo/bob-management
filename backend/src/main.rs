#![allow(
    clippy::multiple_crate_versions,
    clippy::unwrap_used,
    clippy::expect_used
)]

use bob_management::main::prelude::*;

const FRONTEND_FOLDER: &str = "frontend";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = cli::Config::try_from(cli::Args::parse())
        .change_context(AppError::InitializationError)
        .attach_printable("Couldn't get config file.")?;

    let logger = &config.logger;

    let _guard = logger.init_logger().unwrap();
    tracing::info!("Logger: {logger:?}");

    let cors: CorsLayer = config.get_cors_configuration();
    tracing::info!("CORS: {cors:?}");

    let addr = config.address;
    tracing::info!("Listening on {addr}");

    let app = router(cors);
    #[cfg(all(feature = "swagger", debug_assertions))]
    let app = app
        .merge(bob_management::openapi_doc())
        .layer(Extension(RequestTimeout::from(config.request_timeout)));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .change_context(AppError::StartUpError)
        .attach_printable("Failed to start axum server")?;

    Ok(())
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
fn router(cors: CorsLayer) -> Router {
    let session_store = MemoryStore::default();
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            tracing::error!(err);
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_expiry(tower_sessions::Expiry::OnSessionEnd),
        );

    let auth_state = AuthState::new(
        InMemorySessionStore::default(),
        InMemorySessionStore::default(),
    );

    let mut frontend = env::current_exe().expect("Couldn't get current executable path.");
    frontend.pop();
    frontend.push(FRONTEND_FOLDER);
    tracing::info!("serving frontend at: {frontend:?}");
    let router = Router::new()
        // Frontend
        .nest_service("/", ServeDir::new(frontend));

    router
        .nest(
            ApiV1::to_path(),
            api_router_v1(auth_state.clone())
                .expect("couldn't get API routes")
                .layer(ServiceBuilder::new().layer(cors)),
        )
        .layer(session_service)
        .with_state(auth_state)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use bob_management::main::prelude::*;
    use bob_management::services::api_router_v1;
    #[test]
    fn register_routes() {
        let auth_state = AuthState::new(
            InMemorySessionStore::default(),
            InMemorySessionStore::default(),
        );
        let _ = api_router_v1(auth_state).expect("Router has invalid API methods");
    }
}
