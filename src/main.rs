#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

const MANIFEST_RAW: &str = include_str!("../manifest.json");

#[get("/manifest.json")]
fn manifest() -> String {
    MANIFEST_RAW.into()
}

#[get("/catalog/channel/blitz.json")]
fn catalog() -> String {
    format!("nothing")
}

fn main() {
        rocket::ignite().mount("/", routes![catalog]).launch();
}
