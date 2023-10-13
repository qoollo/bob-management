pub mod api;
pub mod bob;
pub mod shared;

pub mod prelude {
    pub use crate::prelude::*;
    pub use hyper::Uri;
    pub use std::{net::SocketAddr, time::Duration};
    pub use tsync::tsync;
}
