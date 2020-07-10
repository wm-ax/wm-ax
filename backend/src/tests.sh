# get post get put get, as in tests.rs
json="Content-Type:application/json"
curl -X GET -H json http://localhost:8000/api -v
article='{"slug" : "theSlug", "content" : "theContent"}'
curl -X POST -H json -d @article.json http://localhost:8000/api/article -v
curl -X POST -H json -d article http://localhost:8000/api/article -v
# curl -X GET -H json http://localhost:8000/api/articles/ahayouga
# newContent="theRevisedContent!!!"
# curl -X PUT -H json -d newContent http://localhost:8000/api
# curl -X GET -H json http://localhost:8000/api/ahayouga
