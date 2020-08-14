table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        slug -> Varchar,
    }
}

table! {
    tags (value) {
        value -> Varchar,
    }
}

table! {
    tags_of_articles (tag_value, article_id) {
        tag_value -> Varchar,
        article_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    tags,
    tags_of_articles,
);
