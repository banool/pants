extern crate serde_json;

use rocket::request::FlashMessage;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    added: Option<String>,
    error: Option<String>,
}

#[get("/")]
pub fn index(flash: Option<FlashMessage>) -> Template {
    let added: Option<String>;
    let error: Option<String>;
    match flash {
        Some(flash) => {
            let msg = flash.message().to_string();
            if flash.kind() == "error" {
                error = Some(msg);
                added = None;
            } else {
                added = Some(msg);
                error = None;
            }
        }
        None => {
            added = None;
            error = None;
        }
    };
    let context = TemplateContext {
        added: added,
        error: error,
    };
    Template::render("index", &context)
}
