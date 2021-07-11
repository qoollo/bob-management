#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use rocket_dyn_templates::tera::{from_value, to_value, Result, Value};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

mod routes;
mod storage;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::get_routes())
        .attach(Template::custom(|engines| {
            engines.tera.register_function("url_for", make_url_for())
        }))
}

fn make_url_for() -> Box<dyn Fn(&HashMap<String, Value>) -> Result<Value> + Send + Sync> {
    let mut route_by_func = HashMap::new();

    for route in routes::get_routes() {
        if let Some(n) = route.name {
            route_by_func.insert(n.to_string(), route.uri.to_string());
        }
    }

    Box::new(move |args| -> Result<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => match route_by_func.get(&v) {
                    Some(uri) => Ok(to_value(uri).unwrap()),
                    None => Err("route not found".into()),
                },
                Err(_) => Err("failed to parse name".into()),
            },
            None => Err("name must be set".into()),
        }
    })
}
