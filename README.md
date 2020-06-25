frameworks
--------------

back-end: REST API with Rocket.rs
front-end: React, Tachyons

design
------

basic concepts are
  * post: can be created and edited by author and by admin
      * can be published/unpublished, maybe also deleted
  * tag: created/edited by admin
  * tab:
  * user role: author/reader/admin

- blog divides into tabs
    - tabs are created/edited/deleted by user
    - each post falls under exactly one tab
    - each tab has its own access properties

api
---

POST
  * create/edit post
  * create/edit tab
  * create/edit/unpublish (author) profile
GET
  * list all posts under a tab, in some order
  * read a given post
  * search for given string
  * list all posts with a given tag
  * read author profile


urls
----

/author_screen_name
/search?p=<search-term>
/<tab_name>/compose
/<tab_name>/<post_title_slug>
/<tab_name>/<post_title_slug>/edit


