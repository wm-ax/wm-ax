extern crate slug;
// use std::borrow::Cow;

use slug::slugify;
// use diesel::pg::PgConnection;
// use diesel::query_builder::Query;
// use diesel::result::QueryResult;
// use diesel::ExpressionMethods;
// use crate::diesel::RunQueryDsl;


use crate::schema::*;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
pub struct Article {
    id: i32,
    title: String,
    // date: String,
    content: String,
    published: bool,
    slug: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleStarter {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name="articles"]
pub struct NewArticle {
    title: String,
    content: String,
    slug: String,
}

impl From<ArticleStarter> for NewArticle {
    fn from(starter: ArticleStarter ) -> Self {
        NewArticle {title: starter.title.clone(),
                    content: starter.content,
                    slug: slugify(starter.title)}
    }
}

pub struct ArticleData<'a> {
    title: &'a str,
    content: &'a str,
    slug: String,
    tags: Vec<&'a str>,
}

// impl <'a> From<ArticleData<'a>> for NewArticle<'a> {
//     fn from(data: ArticleData<'a> ) -> Self {
//         NewArticle {title: data.title,
//                     content: data.content,
//                     slug: slugify(data.title)}
//     }
// }

// // fn insert_article_into_db<'a>(conn: &PgConnection, article_starter: ArticleStarter)
// //                               -> Article {

// //     diesel::insert_into(articles::table)
// //         .values(&NewArticle::from(article_starter))
// //         .get_result(conn)
// //         .expect("Error saving article")
// // }


// fn save_to_db<'a>(article_data: &ArticleData, conn: &PgConnection) -> QueryResult<()> {
//     let slug = slugify(article_data.title);
//     let new_article = NewArticle::from(&article_data);
//     let article = diesel::insert_into(articles::table)
//         .values((new_article, articles::slug.eq(slug)))
//         .execute(conn).unwrap();
//     article_data.tags.iter().for_each(|tag_value| {
//         diesel::insert_into(tags::table)
//             .values(NewTag {value: tag_value.to_string()})
//             .execute(conn);
//     });

//     Ok(())
// }


#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
pub struct Tag {
    value: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag {
    value: String,
}



pub struct TagOfArticle {
    article_id: i32,
    tag_value: String,
}


#[derive(Insertable)]
#[table_name="tags_of_articles"]
struct NewTagOfArticle {
    article_id: i32,
    tag_value: String,
}

