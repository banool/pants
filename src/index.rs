extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {}

pub fn index() -> Template {
    let context = TemplateContext {};
    Template::render("index", &context)
}
