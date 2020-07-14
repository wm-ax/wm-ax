#![feature(proc_macro_hygiene, decl_macro)]

extern crate itertools;

use itertools::Itertools;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate slug;

#[cfg(test)] mod tests;

use std::sync::Mutex;
use std::collections::HashMap;
use slug::slugify;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::content;
// use serde_json::json;

use rocket::http::Method;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};






const LIMIT: u64 = 256;



#[derive(Serialize, Deserialize, Clone, Debug)]
struct Article {
    title: String,
    slug: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArticleStarter {
    title: String,
    content: String,
}


impl From<ArticleStarter>  for Article {
    fn from(starter: ArticleStarter) -> Self {
        Article {title: starter.title.clone(),
                 content: starter.content,
                 slug: slugify(starter.title)}
    }
}

// impl Article {
//     // pub fn new(title: String, content: String) -> Article {
//         // Article {title:title.clone(), content, slug:slugify(title)}
//     // }
//     pub fn from(starter: ArticleStarter) {
//         Article {title: starter.title.clone(),
//                  content,
//                  slug: slugify(title)}
//     }
// }





type ArticleMap = Mutex<HashMap<String, Article>>;


#[post("/article", data = "<article>")]
fn article_create(article: Json<ArticleStarter>, map: State<ArticleMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock");
    let slug = slugify(&article.title);
    print!("received this article: {:?}", article);
    if hashmap.contains_key(&slug) {
        json!({
            "status": "error",
            "reason": "an article with that (slugified) title already exists",
        })
    } else {
        hashmap.insert(slug, article.0.into());
        json!({"status": "ok"})
    }
}


#[put("/article", format = "json", data="<article>")]
fn article_edit(article: Json<Article>, map: State<ArticleMap>)
                -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    let slug = slugify(article.title.clone());
    if hashmap.contains_key(&slug) { 
        hashmap.insert(slug.to_string(), article.0);
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
    print!("ARTICLE_DETAIL!");
    let hashmap = map.lock().unwrap();
    // hashmap.insert("dummytitle1".to_string(), "thedummycontent1".to_string());
    let article = hashmap.get(&slug)
        .map(|article| {Json(article.clone())}
        );
    print!("{:?}", article);
    article
}


// #[get("/")]
// fn article_list() -> content::Json<&'static str> {
//     content::Json("{ 'hi': 'world' }")
// }

#[get("/article")]
fn article_list(map: State<ArticleMap>) -> Json<Vec<Article>> {

    let hashmap = map.lock().unwrap();

    let articles = hashmap.values()
        .map(|article| { article.clone() })
        .collect::<Vec<Article>>();
    print!("{:?}", articles);
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





fn main() -> Result<(), Error> {

   let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter()
            .map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    
    rocket::ignite()
        .mount("/api", routes![
            // search,
            article_create, article_edit, article_detail, article_list,
        ])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<String, Article>::new()))
        .attach(cors)
        .launch();

    Ok(())

}


