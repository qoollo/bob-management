#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use rocket_dyn_templates::Template;

mod routes;
mod storage;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::get_routes())
        .attach(Template::fairing())
}
