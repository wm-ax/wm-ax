CREATE TABLE tags_of_articles (
       tag_value VARCHAR,
       article_id SERIAL,
       PRIMARY KEY (tag_value, article_id)
)
