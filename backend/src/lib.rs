#![allow(clippy::multiple_crate_versions)]

pub mod connector;
pub mod error;
pub mod models;
pub mod services;

// [TEMP]
#[allow(clippy::unused_async)]
#[utoipa::path(
        get,
        path = "/",
        responses(
            (status = 200, description = "Hello Bob!")
        )
    )]
pub async fn root() -> &'static str {
    "Hello Bob!"
}

pub mod prelude {
    #![allow(unused_imports)]
    // pub use crate::bob_server::dto::macros::*;
    pub use crate::error::{APIError, AppError};
    pub use axum::response::Result as AxumResult;
    pub use color_eyre::eyre::{eyre, Report, Result, WrapErr};
}
