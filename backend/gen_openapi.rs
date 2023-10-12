use std::fs;
use utoipa::OpenApi;
// in ./src/gen_openapi.rs
fn main() {
    let doc = backend::ApiDoc::openapi().to_yaml().unwrap();
    let _ = fs::write("./api/openapi.yaml", doc);
}
