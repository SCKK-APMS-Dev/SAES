use std::env;

pub async fn mysql() {
    let url = env::var("DATABASE_URL").unwrap();
}
