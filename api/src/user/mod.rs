use rocket::Route;

pub mod home;

pub fn routes() -> Vec<Route> {
    return routes![home::user_main];
}
