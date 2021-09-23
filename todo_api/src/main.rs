use std::{fs::File, io::BufReader};

use actix_web::{
    delete, get, patch, post, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};

const TODO_FILE_NAME: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    body: String,
}

#[get("/")]
async fn greet(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/todos")]
async fn todo_index(_req: HttpRequest) -> Result<HttpResponse> {
    let file = File::open(TODO_FILE_NAME)?;
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn todo_create(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("unimplemented!")
}

#[patch("/todos/{id}")]
async fn todo_update(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("unimplemented!")
}

#[delete("/todos/{id}")]
async fn todo_delete(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("unimplemented!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
