#[cfg(feature = "frontend")]
mod frontend {
    include!("frontend/build.rs");
}

#[allow(clippy::unwrap_used)]
fn main() {
    #[cfg(feature = "frontend")]
    {
        frontend::build_types();
        frontend::build_frontend();
    }
}
