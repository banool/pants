extern crate rocket_contrib;
extern crate serde_json;

use rocket::request::{Form, FromForm};
use rocket::response::{Flash, Redirect};
use std::env;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize)]
struct TemplateContext {
    url: String,
}

#[derive(FromForm)]
pub struct PocketAddForm {
    title: String,
    tags: String, // Comma separated.
}

#[post("/add", data = "<form>")]
pub fn add(form: Form<PocketAddForm>) -> Flash<Redirect> {
    assert_eq!(form.title.is_empty(), false);

    let title: String = form
        .title
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .collect();
    let title = format!("{}.html", title);

    let created_path = create_page_file(&title);

    let url = format!("{}/{}", &env::var("PANTS_SITE").unwrap(), title);

    Flash::success(Redirect::to("/"), url)
}

fn create_page_file(title: &str) -> String {
    // Creates a file, returns the path of the file it made.
    let mut path = PathBuf::new();
    path.push(&env::var("PANTS_PAGES_ROOT").unwrap());
    path.push(title);
    File::create(path.as_path()).unwrap();
    path.as_path().display().to_string()
}
