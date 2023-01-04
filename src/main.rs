use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/**
 * 1) `/` should return data about the server ✅
 * 2) GET /todos
 * 3) POST /todos
 * 4) DLETE /todos
 * 5) POST /todos
 */

#[derive(Serialize)]
struct AppInfo<'a> {
    app_name: &'a str,
    description: &'a str,
    author: &'a str,
    email: &'a str,
    github: &'a str,
    repo: &'a str,
}

#[derive(Serialize)]
struct ErrorRes {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    desc: String,
    is_done: bool,
}

struct AppState {
    todos: Mutex<HashMap<String, Vec<Todo>>>,
}

#[get("/")]
async fn index() -> impl Responder {
    web::Json(AppInfo {
        app_name: "Memorable",
        description: "Rust based CRUD API for managing todos list. Intended as a testing API for people who just started working with JS fetch API",
        author: "Ahmed Ibrahim",
        email: "me@ahmedibrahim.dev",
        github: "https://github.com/AhmedIbrahim336/",
        repo: "https://github.com/AhmedIbrahim336/memorable"
    })
}

#[post("/todos")]
async fn create_todo(
    req: HttpRequest,
    data: web::Data<AppState>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let addr = match req.peer_addr() {
        Some(addr) => addr.ip().to_string(),
        None => {
            return HttpResponse::BadRequest().json(ErrorRes {
                message: String::from("Missing request socket IP address"),
            })
        }
    };

    let mut todos_map = data.todos.lock().unwrap();
    let todos = (*todos_map).entry(addr.clone()).or_insert(Vec::new());
    todos.push(todo.into_inner());

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                todos: Mutex::new(HashMap::new()),
            }))
            .service(index)
            .service(create_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
