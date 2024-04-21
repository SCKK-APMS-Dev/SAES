use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

mod cucc;
mod db;
mod user;

use user::home::user_main;

#[get("/")]
async fn index() -> impl Responder {
    "SCKK API Szerver v2.0 Actix Web használatával"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(user_main)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
