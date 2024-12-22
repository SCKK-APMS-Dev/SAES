use sea_orm::{EntityTrait, Set};

use crate::{db::logs, utils::sql::get_db_conn};

pub async fn db_log(owner: String, id: Option<i32>, action: &str, message: Option<String>) {
    let db = get_db_conn().await;
    let amodel = logs::ActiveModel {
        owner: Set(owner),
        item_id: Set(id),
        action: Set(String::from(action)),
        message: Set(message),
        ..Default::default()
    };
    logs::Entity::insert(amodel)
        .exec(&db)
        .await
        .expect("[ERROR] Log létrehozása sikertelen");
}
