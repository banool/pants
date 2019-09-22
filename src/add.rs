use rocket::request::{FromForm,Form};

#[derive(FromForm)]
pub struct PocketAddForm {
    title: String,
    tags: String,  // Comma separated.
}

#[post("/add", data = "<pocket_add_form>")]
pub fn add(pocket_add_form: Option<Form<PocketAddForm>>) -> String {
    let url: String = format!("https://dport.me/pages/{}", "hey");
    format!("Created and added {} to Pocket successfully", url)
}
