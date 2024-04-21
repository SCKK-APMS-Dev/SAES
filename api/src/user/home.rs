use crate::cucc::sql;

#[get("/")]
pub fn user_main() -> &'static str {
    "Helloaser, world!"
}
