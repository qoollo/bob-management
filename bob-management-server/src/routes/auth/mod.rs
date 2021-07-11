use rocket::Route;

pub mod login;

pub fn get_routes() -> Vec<Route> {
    routes![login::login, login::login_user]
}
