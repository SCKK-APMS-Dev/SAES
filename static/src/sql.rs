use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn get_db_conn() -> DatabaseConnection {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL .env-ben nem létezik");
    return Database::connect(url).await.unwrap();
}
