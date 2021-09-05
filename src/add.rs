extern crate rocket_contrib;
extern crate serde_json;

use reqwest::blocking::Client as ReqwestClient;
use rocket::request::{Form, FromForm};
use rocket::response::{Flash, Redirect};
use log::info;
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
    let url = format!("{}/pages/{}", &env::var("PANTS_SITE").unwrap(), title);

    match create_page_file(&title) {
        Err(e) => return Flash::error(Redirect::to("/"), format!("Couldn't make file: {}", e)),
        Ok(()) => {}
    }
    match add_to_pocket(&form.title, &url, &form.tags) {
        Err(e) => return Flash::error(Redirect::to("/"), format!("Couldn't add to pocket: {}", e)),
        Ok(()) => {}
    }

    Flash::success(Redirect::to("/"), url)
}

fn create_page_file(title: &str) -> std::io::Result<()> {
    // Creates a file.
    let mut path = PathBuf::new();
    path.push(&env::var("PANTS_PAGES_ROOT").unwrap());
    path.push(title);
    info!("Trying to create {}", path.as_path().display().to_string());
    File::create(path.as_path())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct PocketPostRequest {
    url: String,
    title: String,
    tags: String,
    consumer_key: String,
    access_token: String,
}

fn add_to_pocket(title: &str, url: &str, tags: &str) -> Result<(), &'static str> {
    let request = PocketPostRequest {
        url: url.to_string(),
        title: title.to_string(),
        tags: tags.to_string(),
        consumer_key: env::var("PANTS_CONSUMER_KEY").unwrap(),
        access_token: env::var("PANTS_ACCESS_TOKEN").unwrap(),
    };
    info!("Request: {:#?}", request);
    let client = ReqwestClient::new();
    let response = client
        .post("https://getpocket.com/v3/add")
        .json(&request)
        .send()
        .unwrap();
    info!("Response from Pocket: {:#?}", response);
    let status = response.status();
    if status != 200 {
        return Err("Non-200 status code from Pocket");
    }
    info!("Successfully added {} to pocket", title);
    Ok(())
}
