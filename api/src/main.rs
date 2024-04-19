use std::env;

use dotenv::dotenv;

use sqlx::mysql::MySqlConnection;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[test]
fn test() {
    dotenv().ok();
    println!("Argument: {}", env::var("DATABASE_URL").unwrap());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
