use rocket::Route;

pub mod auth;
pub mod cluster;

pub fn get_routes() -> Vec<Route> {
    vec![auth::get_routes(), cluster::get_routes()]
        .into_iter()
        .flatten()
        .collect()
}
