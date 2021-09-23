use std::{env, fs::File, io::BufReader};

use actix_web::{
    delete, get, middleware, patch, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use serde::{Deserialize, Serialize};

const TODO_FILE_NAME: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    body: String,
}

#[derive(Deserialize, Debug)]
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
