#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
use rocket::{get, routes};
use rocket::http::RawStr;
use std::fs;

#[derive(Deserialize, Debug)]
#[derive(FromForm)]
struct OaiQuery {
    verb: Option<String>,
    #[serde(rename = "metadataPrefix")]
    metadata_prefix: Option<String>,
    set: Option<String>,
}

#[get("/?<verb>")]
fn oai(verb: &RawStr) -> String {
    let filename = format!("data/{}", verb.as_str());
    let contents = fs::read_to_string(filename.to_lowercase());
    contents.unwrap_or("not found".to_string())
}

fn main() -> () {
    rocket::ignite().mount("/", routes![oai]).launch();
}
