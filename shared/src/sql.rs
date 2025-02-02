
use std::{env, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub async fn get_db_conn() -> DatabaseConnection {
    let url = env::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(url).to_owned();
    opt.max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(15))
        .sqlx_logging(true);
    let db_connection = Database::connect(opt).await;
    if db_connection.is_ok() {
        return db_connection.unwrap();
    }
    panic!("Failed to connect to database");
}

pub async fn test_db_conn() {
    get_db_conn().await;
}
