extern crate rocket_contrib;
extern crate serde_json;

use rocket::request::{Form, FromForm};
use rocket::response::{Flash, Redirect};

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
    let url: String = format!("https://dport.me/pages/{}", form.title);
    Flash::success(Redirect::to("/"), url)
}
