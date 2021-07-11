use rocket::Route;

pub mod auth;

pub fn get_routes() -> Vec<Route> {
    vec![auth::get_routes()].into_iter().flatten().collect()
}
