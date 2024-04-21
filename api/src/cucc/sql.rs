use std::env;

use sea_orm::{Database, DatabaseConnection, EntityTrait};

use crate::db::data;

pub async fn mysql() {
    let url = env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(url).await.unwrap();
    let data = data::Entity::find().all(&db).await.unwrap();
    for cucc in data {
        println!("Id: {}, Owner: {}", cucc.id, cucc.owner)
    }
}
