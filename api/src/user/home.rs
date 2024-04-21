use crate::cucc::sql;

#[get("/")]
pub async fn user_main() -> &'static str {
    sql::mysql().await;
    return "Helloaser, world!";
}
