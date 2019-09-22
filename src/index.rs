extern crate rocket_contrib;
extern crate serde_json;

use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    added: Option<String>,
}

#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let added: Option<String>;
    match flash {
        Some(flash_message) => added= Some(flash_message.msg().to_string()),
        None => added = None,
    }
    let context = TemplateContext { added: added };
    Template::render("index", &context)
}
