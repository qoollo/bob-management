use std::fs;
use utoipa::OpenApi;
// in ./src/gen_openapi.rs
fn main() {
    #[derive(OpenApi)]
    #[openapi(
                paths(backend::root),
                tags(
                    (name = "bob", description = "BOB management API")
                )
            )]
    struct ApiDoc;
    let doc = ApiDoc::openapi().to_pretty_json().unwrap();
    let _ = fs::write("./api/openapi.json", doc);
    let doc = ApiDoc::openapi().to_yaml().unwrap();
    let _ = fs::write("./api/openapi.yaml", doc);
}
