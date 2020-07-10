frameworks
--------------

back-end: REST API with Rocket.rs
front-end: React, Tachyons





design (0.1)
============

  * Article: can be created and edited by author and by admin
      * can be published/unpublished, maybe also deleted

views
-----

for posts:
    * create
    * edit
    * read
    * list


api (0.1)
---

root: "/api/articles"

list:
    method: GET
    url: "/"
compose: 
    method: POST
    url: "/"
edit:
    method: PUT
    url: "/<slug>"
detail:
    method: GET
    url: "/<slug>"

user-facing urls
----

list: "/"
compose: "/compose"
edit: "/edit/<slug>"
detail: "/<slug>"







frameworks
--------------

back-end: REST API with Rocket.rs
front-end: React, Tachyons

design
------

basic concepts are
  * Article: can be created and edited by author and by admin
      * can be published/unpublished, maybe also deleted
  * tag: created/edited by admin
  * tab:
  * user role: author/reader/admin

- blog divides into tabs
    - tabs are created/edited/deleted by user
    - each article falls under exactly one tab
    - each tab has its own access properties

api
---

POST
  * create article
  * create tab
  * create/edit/unpublish (author) profile
PUT
  * edit article
  * edit tab
GET
  * list all articles under a tab, in some order
  * read a given article
  * search for given string
  * list all articles with a given tag
  * read author profile


user-facing urls
----

/author_screen_name
/search?p=<search-term>
/<tab_name>/compose
/<tab_name>/<article_title_slug>
/<tab_name>/<article_title_slug>/edit


