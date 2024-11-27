use sea_orm::{EntityTrait, Set};

use crate::{db::logs, utils::sql::get_db_conn};

pub async fn log_update(owner: String, id: i32, tip: String, action: String) {
    let db = get_db_conn().await;
    let amodel = logs::ActiveModel {
        owner: Set(owner),
        r#type: Set(tip),
        item_id: Set(id),
        action: Set(format!("UPDATE{}", action)),
        ..Default::default()
    };
    logs::Entity::insert(amodel)
        .exec(&db)
        .await
        .expect("[ERROR] Log létrehozása sikertelen");
}

pub async fn log_create(owner: String, id: i32, tip: String) {
    let db = get_db_conn().await;
    let amodel = logs::ActiveModel {
        owner: Set(owner),
        r#type: Set(tip),
        item_id: Set(id),
        action: Set(String::from("CREATED")),
        ..Default::default()
    };
    logs::Entity::insert(amodel)
        .exec(&db)
        .await
        .expect("[ERROR] Log létrehozása sikertelen");
}
