extern crate slug;
use slug::slugify;

use crate::schema::*;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
pub struct Article {
    id: i32,
    title: String,
    content: String,
    published: bool,
    slug: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleStarter<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name="articles"]
pub struct NewArticle<'a> {
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

