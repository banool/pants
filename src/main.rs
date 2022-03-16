#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod add;
mod index;

use std::env;
use std::fs::File;

const REQUIRED_ENV_VARS: &'static [&'static str] = &[
    "PANTS_PAGES_ROOT",
    "PANTS_SITE",
    "PANTS_CONSUMER_KEY",
    "PANTS_ACCESS_TOKEN",
    "PANTS_STATIC_ROOT",
];

#[launch]
fn mymain() -> _ {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            panic!("This env var must be set: {}", s);
        }
    }
    let pages_root = &env::var("PANTS_PAGES_ROOT").unwrap();
    File::open(pages_root).expect(format!("The path \"{}\" doesn't exist", pages_root).as_str());

    rocket::build()
        .mount("/", routes![index::index])
        .mount("/", routes![add::add])
        .mount(
            "/static",
            FileServer::from(&env::var("PANTS_STATIC_ROOT").unwrap()),
        )
        .attach(Template::fairing())
}
