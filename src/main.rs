#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
// extern crate slug;

use std::sync::Mutex;
use std::collections::HashMap;
use slug::slugify;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
// use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Article {
    slug: String,
    content: String
}

type ArticleMap = Mutex<HashMap<String, String>>;


// API

#[post("/api/article", format = "json", data = "<article>")]
fn article_create(article: Json<Article>, map: State<ArticleMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock");
    let slug = &article.slug;
    if hashmap.contains_key(slug) {
        json!({
            "status": "error",
            "reason": "an article with that title already exists",
        })
    } else {
            hashmap.insert(slug.clone(), article.content.clone());
        json!({"status": "ok"})
    }
}



#[put("/api/article", format = "json", data="<article>")]
fn article_edit(article: Json<Article>, map: State<ArticleMap>)
                -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    let slug = &article.slug;
    if hashmap.contains_key(slug) {
        hashmap.insert(slug.clone(), article.content.clone());
        Some(json!({"status": "ok"})) }
    else {
        None
    }
}


#[get("/api/<slug>", format = "json")]
fn article_detail(slug: String, map: State<ArticleMap>) -> Option<Json<Article>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&slug).map(
        |content| {
            // Json(Article::from(article.clone()))
            Json(Article{slug, content:content.clone()})
        })
}


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "nothing here."
    })
}


// #[post("/api/<tab_slug>/<article_slug>/edit", format = "json")]
// fn article_submit(tab_slug: String, article_slug: String) -> JsonValue {
//     json!({})
// }

#[get("/api/search?<q>")]
fn search(q: String) -> JsonValue {
    json!({q.clone(): q.clone()})
}

fn main() {
    rocket::ignite().mount("/", routes![
        index,
        search,
        article_create, article_edit, article_detail,
    ]).launch();
}




#[get("/")]
fn index() -> &'static str {
    // react app goes here
    "hi guys!"
}
