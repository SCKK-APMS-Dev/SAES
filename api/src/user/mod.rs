use actix_web::services;
mod home;

pub fn routes() -> (home::user_main,) {
    services![home::user_main]
}
