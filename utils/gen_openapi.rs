use std::fs;
use utoipa::OpenApi;
fn main() {
    let doc = bob_management::ApiDoc::openapi().to_yaml().unwrap();
    let _ = fs::write("./api/openapi.yaml", doc);
}
