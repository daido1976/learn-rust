# todo api

Todo app JSON API.

```sh
$ curl -X GET http://localhost:8080/todos/ | jq .
$ curl -X POST http://localhost:8080/todos/ -d '{ "title": "Test", "body": "bodyだよ" }' | jq .
$ curl -X PUT http://localhost:8080/todos/1 -d '{ "title": "Test2", "body": "updateしたよ" }' | jq .
$ curl -X DELETE http://localhost:8080/todos/1 | jq .
```
