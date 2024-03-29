#![feature(proc_macro_hygiene, decl_macro)]

use lazy_static::*;
use rocket::*;
use rocket_contrib::json::Json;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use stremio_core::types::addons::*;
use stremio_core::types::*;

const TYPE_STR: &str = "channel";
const BLITZ_BASE: &str = "https://www.blitz.bg";
const INVALID_ID: &str = "blitz-invalid-id";

lazy_static! {
    static ref GENRES: Vec<(String, String)> =
        serde_json::from_str(include_str!("../genres_map.json")).unwrap();
}

const MANIFEST_RAW: &str = include_str!("../manifest.json");

#[get("/manifest.json")]
//#[response(content_type = "json")]
fn manifest() -> String {
    MANIFEST_RAW.into()
}

#[get("/catalog/channel/blitz.json")]
fn catalog() -> Option<Json<ResourceResponse>> {
    // @TODO error handling
    Some(Json(
        scrape_blitz(&GENRES[0].0)
            .map(|metas| ResourceResponse::Metas { metas })
            // @TODO fix the unwrap
            .ok()?,
    ))
}

#[get("/catalog/channel/blitz/<genre>")]
fn catalog_genre(genre: String) -> Option<Json<ResourceResponse>> {
    // @TODO from name
    let genre = GENRES.iter().find(|(id, _)| id == &genre)?;
    Some(Json(
        scrape_blitz(&genre.0)
            .map(|metas| ResourceResponse::Metas { metas })
            // @TODO fix the unwrap
            .ok()?,
    ))
}

fn scrape_blitz(genre: &str) -> Result<Vec<MetaPreview>, Box<dyn Error>> {
    let url = format!("{}/{}", BLITZ_BASE, genre);
    let resp = reqwest::get(&url)?;
    if !resp.status().is_success() {
        return Err("request was not a success".into());
    };

    Ok(Document::from_read(resp)?
        .find(Name("article"))
        .filter_map(|article| {
            // if we cannot find name, we're probably finding the wrong items
            let name = get_name_from_article(&article)?;
            Some(MetaPreview {
                id: get_id_from_article(&article).unwrap_or_else(|| INVALID_ID.to_owned()),
                type_name: TYPE_STR.to_owned(),
                poster: Some(get_poster_from_article(&article)?),
                name,
                poster_shape: PosterShape::Landscape,
            })
        })
        .collect())
}

fn get_id_from_article(article: &select::node::Node) -> Option<String> {
    article
        .find(Name("a"))
        .next()?
        .attr("href")
        .map(|s| s.split('/').skip(3).collect::<Vec<&str>>().join("/"))
}

fn get_poster_from_article(article: &select::node::Node) -> Option<String> {
    let elem = article.find(Name("img")).next()?;
    elem.attr("src")
        .or_else(|| elem.attr("data-original"))
        .map(|s| s.to_owned())
}

fn get_name_from_article(article: &select::node::Node) -> Option<String> {
    Some(article.find(Name("h3")).next()?.text().trim().to_string())
}

fn main() {
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .mount("/", routes![manifest, catalog, catalog_genre])
        .attach(cors)
        .launch();
}
