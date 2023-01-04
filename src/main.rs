use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
/**
 * 1) `/` should return data about the server
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

struct AppState {
    app_name: String,
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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("TODO_API"),
            }))
            .service(index)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
