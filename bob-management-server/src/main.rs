#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("dummy", true);
    Template::render("index/index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
