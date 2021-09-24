use std::{env, fs::File, io::BufReader};

use actix_web::{
    delete, get, middleware, patch, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

const TODO_FILE_NAME: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Todo {
    id: u32,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoParams {
    title: String,
    body: String,
}

/// fetch current todos from json file
fn fetch_current_todos() -> Result<Vec<Todo>> {
    let file = File::open(TODO_FILE_NAME)?;
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(todos)
}

/// persist by writing todos to json file
fn persist(todos: &[Todo]) -> Result<()> {
    let file = File::create(TODO_FILE_NAME)?;
    serde_json::to_writer_pretty(&file, todos)?;
    Ok(())
}

#[get("/")]
async fn greet(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/todos")]
async fn todo_index(_req: HttpRequest) -> Result<HttpResponse> {
    let todos = fetch_current_todos()?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn todo_create(params: web::Json<TodoParams>) -> Result<HttpResponse> {
    let mut todos = fetch_current_todos()?;
    let param_todo = params.into_inner();

    // build new_todos
    let new_id = todos.len() + 1;
    let new_todo = Todo {
        id: new_id as u32,
        title: param_todo.title,
        body: param_todo.body,
    };
    todos.push(new_todo.clone());

    persist(&todos)?;
    Ok(HttpResponse::Ok().json(new_todo))
}

#[patch("/todos/{id}")]
async fn todo_update(
    web::Path(id): web::Path<u32>,
    params: web::Json<TodoParams>,
) -> Result<HttpResponse> {
    let mut todos = fetch_current_todos()?;
    let param_todo = params.into_inner();

    // update todos
    todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| {
            todo.title = param_todo.title;
            todo.body = param_todo.body;
        })
        .unwrap();

    persist(&todos)?;

    // find updated todo
    let updated_todo = todos.iter().find(|todo| todo.id == id).unwrap();
    Ok(HttpResponse::Ok().json(updated_todo))
}

#[delete("/todos/{id}")]
async fn todo_delete(web::Path(id): web::Path<u32>) -> Result<HttpResponse> {
    let mut todos = fetch_current_todos()?;

    // delete todo
    todos.retain(|todo| todo.id != id);

    persist(&todos)?;
    Ok(HttpResponse::Ok().json("{}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .service(greet)
            .service(todo_index)
            .service(todo_create)
            .service(todo_update)
            .service(todo_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use actix_web::{dev::Body, test, App};

    #[actix_rt::test]
    async fn test_crud() {
        // setup json file
        fs::write(TODO_FILE_NAME, "[]").unwrap();

        let mut app = test::init_service(
            App::new()
                .service(todo_index)
                .service(todo_create)
                .service(todo_update)
                .service(todo_delete),
        )
        .await;

        // test index
        let index_req = test::TestRequest::get()
            .header("content-type", "application/json")
            .uri("/todos")
            .to_request();
        let mut index_resp = test::call_service(&mut app, index_req).await;

        // See. https://stackoverflow.com/questions/63910673/how-to-get-the-body-of-a-response-in-actix-web-unit-test
        let resp_body = index_resp.take_body();
        let resp_body = resp_body.as_ref().unwrap();
        let expected_body = &Body::from("[]");
        assert_eq!(index_resp.status(), 200);
        assert_eq!(expected_body, resp_body);

        // これでもいける
        // See. https://docs.rs/actix-web/3.3.2/actix_web/test/fn.read_body_json.html
        // assert_eq!(index_resp.status(), 200);
        // let expected_body: Vec<Todo> = serde_json::from_str("[]").unwrap();
        // let resp_body: Vec<Todo> = test::read_body_json(index_resp).await;
        // assert_eq!(expected_body, resp_body);

        // test create
        let create_req = test::TestRequest::post()
            .header("content-type", "application/json")
            .uri("/todos")
            .set_json(&TodoParams {
                title: "test title".to_string(),
                body: "test body".to_string(),
            })
            .to_request();
        let mut create_resp = test::call_service(&mut app, create_req).await;
        let resp_body = create_resp.take_body();
        let resp_body = resp_body.as_ref().unwrap();
        let expected_body = &Body::from(
            serde_json::to_string(&Todo {
                id: 1,
                title: "test title".to_string(),
                body: "test body".to_string(),
            })
            .unwrap(),
        );

        assert_eq!(create_resp.status(), 200);
        assert_eq!(expected_body, resp_body);

        // test update
        let update_req = test::TestRequest::patch()
            .header("content-type", "application/json")
            .uri("/todos/1")
            .set_json(&TodoParams {
                title: "updated title".to_string(),
                body: "updated body".to_string(),
            })
            .to_request();
        let mut update_resp = test::call_service(&mut app, update_req).await;
        let resp_body = update_resp.take_body();
        let resp_body = resp_body.as_ref().unwrap();
        let expected_body = &Body::from(
            serde_json::to_string(&Todo {
                id: 1,
                title: "updated title".to_string(),
                body: "updated body".to_string(),
            })
            .unwrap(),
        );

        assert_eq!(update_resp.status(), 200);
        assert_eq!(expected_body, resp_body);

        // test index when todo is updated
        let index_req = test::TestRequest::get()
            .header("content-type", "application/json")
            .uri("/todos")
            .to_request();
        let index_resp = test::call_service(&mut app, index_req).await;
        assert_eq!(index_resp.status(), 200);

        // See. https://docs.rs/actix-web/3.3.2/actix_web/test/fn.read_body_json.html
        let expected_body: Vec<Todo> = vec![Todo {
            id: 1,
            title: "updated title".to_string(),
            body: "updated body".to_string(),
        }];
        let resp_body: Vec<Todo> = test::read_body_json(index_resp).await;
        assert_eq!(expected_body, resp_body);

        // test delete
        let delete_req = test::TestRequest::delete()
            .header("content-type", "application/json")
            .uri("/todos/1")
            .to_request();
        let delete_resp = test::call_service(&mut app, delete_req).await;

        assert_eq!(delete_resp.status(), 200);
    }
}
