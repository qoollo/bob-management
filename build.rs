#[cfg(feature = "frontend")]
mod frontend {
    include!("frontend/build.rs");
}

fn main() {
    #[cfg(feature = "frontend")]
    {
        frontend::build_types();
        frontend::build_frontend();
    }
}
