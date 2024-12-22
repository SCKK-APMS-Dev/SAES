use sea_orm::EntityTrait;

use crate::db::{statuses, types};

use super::sql::get_db_conn;

pub async fn get_item_status_int(status_text: String) -> Option<i32> {
    let db = get_db_conn().await;
    let stat = statuses::Entity::find()
        .all(&db)
        .await
        .expect("Státuszok lekérése sikertelen");
    for us in stat.iter() {
        if us.name == status_text {
            return Some(us.id);
        }
    }
    None
}

pub async fn get_item_status_string(status_id: i32) -> Option<String> {
    let db = get_db_conn().await;
    let stat = statuses::Entity::find()
        .all(&db)
        .await
        .expect("Státuszok lekérése sikertelen");
    for us in stat.iter() {
        if us.id == status_id {
            return Some(us.name.clone());
        }
    }
    None
}

pub async fn get_item_type_int(item_type: String) -> Option<i32> {
    let db = get_db_conn().await;
    let stat = types::Entity::find()
        .all(&db)
        .await
        .expect("Típusok lekérése sikertelen");
    for us in stat.iter() {
        if us.display == item_type {
            return Some(us.id);
        }
    }
    None
}
pub async fn get_item_type_string(item_id: i32) -> Option<String> {
    let db = get_db_conn().await;
    let stat = types::Entity::find()
        .all(&db)
        .await
        .expect("Típusok lekérése sikertelen");
    for us in stat.iter() {
        if us.id == item_id {
            return Some(us.display.clone());
        }
    }
    None
}
