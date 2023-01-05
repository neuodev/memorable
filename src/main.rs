mod todos;

use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/**
 * 1) `/` should return data about the server ✅
 * 2) GET /todos ✅
 * 3) POST /todos ✅
 * 4) DLETE /todos
 * 5) PUT /todos
 * 6) GET /todos/{id}
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

#[derive(Serialize)]
struct OkResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
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

    HttpResponse::Ok().json(OkResponse {
        message: String::from("Todo created successfully"),
    })
}

#[get("/todos")]
async fn get_todos(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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

    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
async fn get_todo_by_id(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<usize>,
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
    let todos = (*todos_map).entry(addr).or_insert(Vec::new());
    let todo_id = path.into_inner();
    let todo = todos.iter().find(|todo| todo.id == todo_id);

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_data = web::Data::new(AppState {
        todos: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .app_data(app_data.clone())
            .service(
                web::scope("/api/v1")
                    .service(index)
                    .service(create_todo)
                    .service(get_todos)
                    .service(get_todo_by_id),
            )
    })
    .workers(6)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
