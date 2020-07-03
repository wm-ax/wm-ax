#![feature(proc_macro_hygiene, decl_macro)]

extern crate itertools;

use itertools::Itertools;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
// extern crate slug;

#[cfg(test)] mod tests;

use std::sync::Mutex;
use std::collections::HashMap;
// use slug::slugify;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::content;
// use serde_json::json;

use rocket_cors;

#[derive(Serialize, Deserialize)]
struct Article {
    slug: String,
    content: String
}

type ArticleMap = Mutex<HashMap<String, String>>;

// API


#[post("/article", format = "json", data = "<article>")]
fn article_create(article: Json<Article>, map: State<ArticleMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock");
    let slug = &article.slug;
    if hashmap.contains_key(slug) {
        json!({
            "status": "error",
            "reason": "an article with that slug already exists",
        })
    } else {
            hashmap.insert(slug.clone(), article.content.clone());
        json!({"status": "ok"})
    }
}


#[put("/article", format = "json", data="<article>")]
fn article_edit(article: Json<Article>, map: State<ArticleMap>)
                -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    let slug = &article.slug;
    if hashmap.contains_key(slug) { 
        hashmap.insert(slug.clone(), article.content.clone());
        json!({"status": "ok"})
    }
    else {
        {
            json!({"status": "error",
                   "reason": format!("there is no article with the slug {}\n the keys available are {}",
                                     slug,
                                     hashmap.keys().join("\n\t"))})
        }
    }
}


#[get("/article/<slug>")]
fn article_detail(slug: String, map: State<ArticleMap>) -> Option<Json<Article>> {
    let hashmap = map.lock().unwrap();
    // hashmap.insert("dummytitle1".to_string(), "thedummycontent1".to_string());
    hashmap.get(&slug).map(
        |content| {
            // Json(Article::from(article.clone()))
            Json(Article{slug, content:content.clone()})
        }
    )
}


// #[get("/")]
// fn article_list() -> content::Json<&'static str> {
//     content::Json("{ 'hi': 'world' }")
// }

#[get("/article")]
fn article_list(map: State<ArticleMap>) -> Json<Vec<Article>> {

    let hashmap = map.lock().unwrap();

    let articles = hashmap.iter().map(
        |(slug, content)|{
            Article{slug:slug.to_string(), content:content.to_string()}
        }).collect::<Vec<Article>>();
    Json(articles)
}


#[get("/search?<q>")]
fn search(q: String) -> JsonValue {
    json!({q.clone(): q.clone()})
}



#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "nothing here."
    })
}




fn rocket() -> rocket::Rocket {

    
    rocket::ignite()
        .mount("/api", routes![
            search,
            article_create, article_edit, article_detail, article_list,
        ])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<String, String>::new()))
}


fn main() {
    rocket().launch();
    // rocket::ignite().mount("/api", routes![
    //     // index,
    //     search,
    //     article_create, article_edit, article_detail, article_list,
    // ]).launch();
}




// #[get("/")]
// fn index() -> &'static str {
//     // react app goes here
//     "hi guys!"
// }
