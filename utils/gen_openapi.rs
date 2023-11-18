use std::fs;
use utoipa::OpenApi;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to save the OpenAPI schema
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let doc = bob_management::ApiDoc::openapi().to_yaml().unwrap();
    let _ = fs::write(args.filename, doc).expect("Couldn't write schema to file");
}
