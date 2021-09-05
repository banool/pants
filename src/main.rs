#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

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

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            panic!("This env var must be set: {}", s);
        }
    }
    let pages_root = &env::var("PANTS_PAGES_ROOT").unwrap();
    File::open(pages_root).expect(format!("The path \"{}\" doesn't exist", pages_root).as_str());

    rocket::ignite()
        .mount("/", routes![index::index])
        .mount("/", routes![add::add])
        .mount(
            "/static",
            StaticFiles::from(&env::var("PANTS_STATIC_ROOT").unwrap()),
        )
        .attach(Template::fairing())
        .launch();
}
