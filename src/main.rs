#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

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
];

fn main() {
    for s in REQUIRED_ENV_VARS.iter() {
        if env::var(s).is_err() {
            panic!(format!("This env var must be set: {}", s));
        }
    }
    let pages_root = &env::var("PANTS_PAGES_ROOT").unwrap();
    File::open(pages_root).expect(format!("The path {} doesn't exist", pages_root).as_str());

    rocket::ignite()
        .mount("/", routes![index::index])
        .mount("/", routes![add::add])
        .attach(Template::fairing())
        .launch();
}
