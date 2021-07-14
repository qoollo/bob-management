#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use rocket::response::Redirect;
use rocket_dyn_templates::Template;

mod routes;
mod services;
mod storages;

#[get("/")]
fn default() -> Redirect {
    Redirect::to(uri!(routes::auth::login::get))
}

#[launch]
fn rocket() -> _ {
    let mut routes = routes::get_routes();
    routes.extend(routes![default]);
    rocket::build()
        .mount("/", routes)
        .attach(Template::fairing())
}
