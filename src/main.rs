#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate dotenv;

// use diesel::prelude::*;
// use diesel::pg::PgConnection;
// use dotenv::dotenv;
// use std::env;
// // use rocket_contrib::json::{Json, JsonValue};

// use models::*;

// pub mod schema;
// pub mod models;


// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }


#[get("/")]
fn index() -> &'static str {
    "hi guys!"
}

// fn tab_detail(tab_slug: String) -> JsonValue {
//     json!({})
// }

// #[get("<tab_slug>/<article_slug>")]
// fn article_detail(tab_slug: String, article_slug: String) -> JsonValue {
//     json!({})
// }

// #[get("/<tab_slug>/<article_slug>/edit")]
// fn article_edit(tab_slug: String, article_slug: String) -> JsonValue {
//     json!({})
// }

// #[get("/<tab_slug>/compose")]
// fn article_create(tab_slug: String) -> JsonValue {
//     json!({})
// }


// #[get("/search?<q>")]
// fn search(q: String) -> JsonValue {
//     json!({})
// }


#[get("/search?<q>")]
fn search(q: String) -> String {
    format!("you searched for {}", q)
}


fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
