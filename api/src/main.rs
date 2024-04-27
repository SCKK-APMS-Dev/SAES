use actix_web::{
    dev::Service, get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use futures_util::future::FutureExt;

mod cucc;
mod db;
mod user;

#[get("/")]
async fn index() -> impl Responder {
    "SCKK API Szerver v2.0 Actix Web + sqlx használatával"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .default_service(web::to(|| HttpResponse::NotFound()))
            .service(index)
            .service(user::routes().wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            }))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
