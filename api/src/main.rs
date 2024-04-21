#[macro_use]
extern crate rocket;

mod cucc;
mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/user", user::routes())
}
