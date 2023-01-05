mod todos;

use actix_web::{
    delete, get, middleware::Logger, post, put, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use todos::{CreateTodo, Todo, UpdateTodo};

/**
 * 1) `/` should return data about the server ✅
 * 2) GET /todos ✅
 * 3) POST /todos ✅
 * 4) DELETE /todos
 * 5) PUT /todos/{id}
 * 6) GET /todos/{id} ✅
 * Todo: Add middle ware to check for `ip` address
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
struct MsgResponse {
    message: String,
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
    todo_json: web::Json<CreateTodo>,
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
    let todo = todo_json.into_inner();
    todos.push(Todo {
        id: todos.len() + 1,
        title: todo.title,
        desc: todo.desc,
        is_done: todo.is_done,
    });

    HttpResponse::Ok().json(MsgResponse {
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
        None => HttpResponse::NotFound().json(MsgResponse {
            message: String::from("Todo not found"),
        }),
    }
}

#[put("/todos/{id}")]
async fn update_todo(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<usize>,
    req_body: web::Json<UpdateTodo>,
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
    let todo = todos.iter_mut().find(|todo| todo.id == todo_id);

    let found_todo = match todo {
        Some(todo) => todo,
        None => {
            return HttpResponse::NotFound().json(MsgResponse {
                message: String::from("Todo not found"),
            })
        }
    };

    let body = req_body.into_inner();
    found_todo.title = body.title.unwrap_or(found_todo.title.clone());
    found_todo.desc = body.desc.unwrap_or(found_todo.desc.clone());
    found_todo.is_done = body.is_done.unwrap_or(found_todo.is_done.clone());

    HttpResponse::Ok().json(found_todo)
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
                    .service(get_todo_by_id)
                    .service(update_todo),
            )
    })
    .workers(6)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
