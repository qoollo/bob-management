use crate::storage::session_data_storage::*;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::Response;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct NodeDto {
    name: String,
    addr: String,
    active: bool,
}

impl NodeDto {
    fn new(name: &str, addr: &str, active: bool) -> Self {
        NodeDto {
            name: name.to_string(),
            addr: addr.to_string(),
            active,
        }
    }
}

#[derive(Serialize)]
pub struct IndexContext {
    nodes: Vec<NodeDto>,
}

#[get("/cluster")]
pub fn get(cookie_jar: &CookieJar) -> Result<Template, Redirect> {
    if let Some(addr) = cookie_jar.find_cluster_addr() {
        debug!("Addr already set to {:?}!", addr);
        let nodes = vec![
            NodeDto::new("Dummy1", &addr.to_string(), true),
            NodeDto::new("Dummy2", &addr.to_string(), false),
        ];
        Ok(Template::render("cluster/index", IndexContext { nodes }))
    } else {
        Err(Redirect::to(uri!(crate::routes::auth::login::get)))
    }
}