#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use std::error::Error;
use select::document::Document;
use select::predicate::Name;
use stremio_state_ng::types::*;

const TYPE_STR: &str = "channel";
const BLITZ_BASE: &str = "https://www.blitz.bg";
const INVALID_ID: &str = "blitz-invalid-id";
const POSTER_SHAPE: &str = "landscape";

const MANIFEST_RAW: &str = include_str!("../manifest.json");

#[get("/manifest.json")]
fn manifest() -> String {
    MANIFEST_RAW.into()
}

#[get("/catalog/channel/blitz.json")]
fn catalog() -> Json<ResourceResponse> {
    // @TODO: responder
    Json(scrape_blitz()
        .map(|metas| ResourceResponse::Metas{ metas, has_more: false, skip: 0 })
        // @TODO fix the unwrap
        .unwrap())
}

fn scrape_blitz() -> Result<Vec<MetaPreview>, Box<dyn Error>> {
    let url = format!("{}/{}", BLITZ_BASE, "zdrave");
    let resp = reqwest::get(&url)?;
    if !resp.status().is_success() {
        return Err("request was not a success".into());
    };

    Ok(Document::from_read(resp)?
        .find(Name("article"))
        .map(|article| MetaPreview{
            id: get_id_from_article(&article).unwrap_or(INVALID_ID.to_owned()),
            poster: get_poster_from_article(&article),
            type_name: TYPE_STR.to_owned(),
            name: article.text().trim().to_string(),
            poster_shape: Some(POSTER_SHAPE.to_owned()),
        })
        .collect()
    )
}

fn get_poster_from_article(article: &select::node::Node) -> Option<String> {
    article.find(Name("img"))
        .next()?
        .attr("src")
        .map(|s| s.to_owned())
}

fn get_id_from_article(article: &select::node::Node) -> Option<String> {
    article.find(Name("a"))
        .next()?
        .attr("href")
        .map(|s| s.split("/").skip(3).collect::<Vec<&str>>().join("/"))
}

fn main() {
        rocket::ignite().mount("/", routes![catalog]).launch();
}
