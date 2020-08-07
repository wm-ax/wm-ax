#![feature(proc_macro_hygiene, decl_macro)]

// extern crate itertools;

// use itertools::Itertools;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate slug;

#[cfg(test)] mod tests;

// use std::sync::Mutex;
// use std::collections::HashMap;
use slug::slugify;

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


#[derive(Serialize, Deserialize, Clone, Debug)]
struct ArticleStarter<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name="articles"]
struct NewArticle<'a> {
    title: &'a str,
    content: &'a str,
    slug: String,
}

impl <'a> From<ArticleStarter<'a>> for NewArticle<'a> {
    fn from(starter: ArticleStarter<'a> ) -> Self {
        NewArticle {title: starter.title,
                    content: starter.content,
                    slug: slugify(starter.title)}
    }
}




// impl Insertable for ArticleStarter {
//     type Values = <(
//         Option<diesel::dsl::Eq<articles::id, i32>>,
//         Option<diesel::dsl::Eq<articles::title,  Varchar>>,
//         Option<diesel::dsl::Eq<articles::content,  Text>>,
//         Option<diesel::dsl::Eq<articles::published,  Bool>>,
//         Option<diesel::dsl::Eq<articles::slug,  Varchar>>
//     ) as diesel::insertable::Insertable<articles::table>>::Values;


//     fn values(self) -> Self::Values {
//         #[allow(non_shorthand_field_patterns)]
//         let Self { title: ref title,
//                    content: ref content,
//         } = *self;
//         let 
//         diesel::insertable::Insertable::values((
//             Some(::ExpressionMethods::eq(articles::id, id)),
//             Some(::ExpressionMethods::eq(articles::title, title)),            
//             Some(::ExpressionMethods::eq(articles::content, content)),
//             Some(::ExpressionMethods::eq(articles::published, published)),
//             Some(::ExpressionMethods::eq(articles::slug, slug))            
//         ))
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

fn insert_article_into_db<'a>(conn: &PgConnection, article_starter: ArticleStarter)
                              -> Article {
    // use schema::articles;

    diesel::insert_into(articles::table)
        .values(&NewArticle::from(article_starter))
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

    print!("{:?}", article);

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
fn article_detail(sought_slug: String) -> Option<Json<Article>> {

    use schema::articles::dsl::*;

    let connection = establish_connection();
 
    articles.filter(id.eq(1))
        .load::<Article>(&connection)
        .expect("Couldn't load article")
        .get(1).map(|a|Json(a.clone()))
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
            article_create, article_detail, article_list,
        ])
        .register(catchers![not_found])
        .attach(cors)
        .launch();

    Ok(())

}


