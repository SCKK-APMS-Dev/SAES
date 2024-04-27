use actix_web::{services, web};
mod home;

pub fn routes() -> (actix_web::Scope,) {
    services![web::scope("/user").service(services![home::user_main])]
}
