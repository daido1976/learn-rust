use actix_web::{
    get,
    http::header,
    middleware, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, ResponseError,
};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct AddParams {
    text: String,
}
#[derive(Serialize, Deserialize)]
struct DeleteParams {
    id: u32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to render HTML")]
    Askama(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPool(#[from] r2d2::Error),

    #[error("Failed to sql exectionion")]
    Sqlite(#[from] rusqlite::Error),
}
impl ResponseError for MyError {}

#[get("/")]
async fn index(db: Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT * from todos;")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    let html = IndexTemplate { entries };

    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todos (text) VALUES (?);", [&params.text])?;

    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[post("/delete")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todos WHERE id = ?;", [&params.id])?;

    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize");
    let conn = pool.get().expect("Failed to get connection");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        );",
        params![],
    )
    .expect("Failed to create todo table");

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await?;
    Ok(())
}
