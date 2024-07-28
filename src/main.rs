mod middleware;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use middleware::ExampleMiddlewareFactory;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        let middleware = ExampleMiddlewareFactory {};
        App::new().wrap(middleware).service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
