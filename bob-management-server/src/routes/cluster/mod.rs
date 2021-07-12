use rocket::Route;

pub mod index;

pub fn get_routes() -> Vec<Route> {
    routes![index::get]
}
