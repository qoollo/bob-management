use crate::storage::session_data_storage::*;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::BTreeMap;

#[derive(FromForm)]
pub struct LoginUserInput {
    pub cluster_addr: String,
}

#[get("/auth/login")]
pub fn get(cookie_jar: &CookieJar) -> Template {
    let mut context = BTreeMap::new();
    if let Some(addr) = cookie_jar.find_cluster_addr() {
        debug!("Addr already set to {:?}!", addr);
        context.insert("current_cluster_addr", addr.to_string());
    }
    Template::render("auth/login", context)
}

#[post("/auth/login", data = "<input>")]
pub fn post(input: Form<LoginUserInput>, cookie_jar: &CookieJar) -> Redirect {
    debug!("received cluster addr {}", input.cluster_addr);
    if let Ok(addr) = input.cluster_addr.parse() {
        cookie_jar.save_cluster_addr(addr);
    }
    Redirect::to(uri!(crate::routes::cluster::index::get))
}
