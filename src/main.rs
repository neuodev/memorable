use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/**
 * 1) `/` should return data about the server
 * 2) GET /todos
 * 3) POST /todos
 * 4) DLETE /todos
 * 5) POST /todos
 */

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello, {app_name}")
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
