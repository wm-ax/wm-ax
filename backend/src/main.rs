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
// use rocket::response::content;
// use serde_json::json;

use rocket::http::Method;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;



const LIMIT: u64 = 256;


pub mod schema;
// pub mod models;



#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
struct Article {
    id: i32,
    title: String,
    content: String,
    published: bool,
    slug: String,
}


use schema::articles;

// #[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
// #[table_name="articles"]
// struct ArticleStarter {
//     title: str,
//     content: str,
// }


#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name="articles"]
struct ArticleStarter<'a> {
    title: &'a str,
    content: &'a str,
}


// impl <'a> From<ArticleStarter>  for Article {
//     fn from(starter: ArticleStarter) -> Self {
//         Article {title: starter.title.to_string(),
//                  content: starter.content,
//                  slug: slugify(starter.title)}
//     }
// }






pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



// type ArticleMap = Mutex<HashMap<String, Article>>;

fn insert_article_into_db<'a>(conn: &PgConnection, new_article: ArticleStarter)
                              -> Article {
    // use schema::articles;

    // let new_article = ArticleStarter {
    //     title: title,
    //     content: content,
    // };

    diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(conn)
        .expect("Error saving article")
}

#[post("/article", data = "<new_article>")]
fn article_create(new_article: Json<ArticleStarter>)
                  -> JsonValue {

    let conn = establish_connection();

    // let article = diesel::insert_into(articles::table)
    //     .values(&new_article)
    //     .get_result(&conn)
    //     .expect("Error saving article");

    let article = insert_article_into_db(&conn, new_article.0.into());

    json!({"status": "Ok"})
    // let mut hashmap = map.lock().expect("map lock");
    // let slug = slugify(&article.title);
    // print!("received this article: {:?}", article);
    // if hashmap.contains_key(&slug) {
    //     json!({
    //         "status": "error",
    //         "reason": "an article with that (slugified) title already exists",
    //     })
    // } else {
    //     // hashmap.insert(slug, article.0.into());
    //     json!({"status": "ok"})
    // }

}

// #[post("/article", data = "<article>")]
// fn article_create(article: Json<ArticleStarter>, map: State<ArticleMap>) -> JsonValue {
//     let mut hashmap = map.lock().expect("map lock");
//     let slug = slugify(&article.title);
//     print!("received this article: {:?}", article);
//     if hashmap.contains_key(&slug) {
//         json!({
//             "status": "error",
//             "reason": "an article with that (slugified) title already exists",
//         })
//     } else {
//         // hashmap.insert(slug, article.0.into());
//         json!({"status": "ok"})
//     }
// }






// #[put("/article", format = "json", data="<article>")]
// fn article_edit(article: Json<Article>)
//                 -> JsonValue {
//     let mut hashmap = map.lock().unwrap();
//     let slug = slugify(article.title.clone());
//     if hashmap.contains_key(&slug) { 
//         hashmap.insert(slug.to_string(), article.0);
//         json!({"status": "ok"})
//     }
//     else {
//         {
//             json!({"status": "error",
//                    "reason": format!("there is no article with the slug {}\n the keys available are {}",
//                                      slug,
//                                      hashmap.keys().join("\n\t"))})
//         }
//     }
// }


#[get("/article/<sought_slug>")]
fn article_detail(sought_slug: String) -> Json<Article> {

    use schema::articles::dsl::*;

    let connection = establish_connection();

    let article = articles.filter(id.eq(1))
        .load::<Article>(&connection)
        .expect("Couldn't load article");

    Json(article[1].clone())
}


#[get("/article")]
fn article_list() -> Json<Vec<Article>> {

    use schema::articles::dsl::*;

    let connection = establish_connection();
    let results = articles
        .filter(published.eq(true))
        // .limit(5)
        .load::<Article>(&connection)
        .expect("Error loading posts");
        
    Json(results)
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
        .attach(cors)
        .launch();

    Ok(())

}


