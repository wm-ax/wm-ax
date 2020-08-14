#![feature(proc_macro_hygiene, decl_macro)]

// extern crate itertools;
// use itertools::Itertools;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] 
mod tests;

// use rocket::State;
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
pub mod models;

use models::*;
use schema::*;
// use models::{Article, Tag, TagOfArticle, ArticleStarter};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}




fn insert_article_into_db<'a>(conn: &PgConnection, article_starter: ArticleStarter)
                              -> Article {

    diesel::insert_into(articles::table)
        .values(&NewArticle::from(article_starter))
        .get_result(conn)
        .expect("Error saving article")
}


#[post("/article", data = "<new_article>")]
fn article_create(new_article: Json<ArticleStarter>)
                  -> JsonValue {

    let conn = establish_connection();

    let article = insert_article_into_db(&conn, new_article.0.into());

    print!("{:?}", article);

    json!({"status": "Ok"})

}



#[get("/article/<sought_slug>")]
fn article_detail(sought_slug: String) -> Option<Json<Article>> {

    use schema::articles::dsl::*;

    let connection = establish_connection();
 
    print!("sought slug: {}\n", sought_slug);
    let a = articles.filter(slug.eq(sought_slug))
        .load::<Article>(&connection)
        .expect("Couldn't load article")
        .get(0).map(|a|Json(a.clone()));
    print!("{:?}", a);
    return a;
}


#[get("/article")]
fn article_list() -> Json<Vec<Article>> {

    use schema::articles::dsl::*;

    let connection = establish_connection();
    let results = articles
        // .filter(published.eq(true))
        .load::<Article>(&connection)
        .expect("Error loading posts");
        
    let ret = Json(results);
    print!("returned the following results: {:?}", ret);
    ret
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
            article_create, article_detail, article_list,
        ])
        .register(catchers![not_found])
        .attach(cors)
        .launch();

    Ok(())

}


