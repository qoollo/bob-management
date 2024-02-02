pub mod api;
pub mod bob;
pub mod shared;

pub mod prelude {
    pub use crate::prelude::*;
    pub use hyper::Uri;
    pub use std::{
        net::SocketAddr,
        ops::{Add, AddAssign},
        time::Duration,
    };
    pub use strum::{EnumIter, IntoEnumIterator};
    pub use tsync::tsync;
    #[cfg(all(feature = "swagger", debug_assertions))]
    pub use utoipa::openapi::{Object, ObjectBuilder};
}
