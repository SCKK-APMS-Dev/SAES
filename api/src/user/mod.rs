use actix_web::web;

mod home;

pub fn routes() -> actix_web::Scope {
    web::scope("/user").service(home::user_main)
}
