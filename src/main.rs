#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::templates::Template;

mod add;
mod index;

fn main() {
    rocket::ignite()
        .mount("/", routes![index::index])
        .mount("/", routes![add::add])
        .attach(Template::fairing())
        .launch();
}
