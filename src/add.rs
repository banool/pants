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
    let url = format!("{}/pages/{}.html", &env::var("PANTS_SITE").unwrap(), title);

    create_page_file(&title);
    add_to_pocket(&form.title, &url, &form.tags);

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

#[derive(Debug, Serialize, Deserialize)]
struct PocketPostRequest {
    url: String,
    title: String,
    tags: String,
    consumer_key: String,
    access_token: String,
}

fn add_to_pocket(title: &str, url: &str, tags: &str) {
    let request = PocketPostRequest {
        url: url.to_string(),
        title: title.to_string(),
        tags: tags.to_string(),
        consumer_key: env::var("PANTS_CONSUMER_KEY").unwrap(),
        access_token: env::var("PANTS_ACCESS_TOKEN").unwrap(),
    };
    let client = reqwest::Client::new();
    let response = client
        .post("https://getpocket.com/v3/add")
        .json(&request)
        .send()
        .unwrap();
    println!("{:#?}", response.status());
}
