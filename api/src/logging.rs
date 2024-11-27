use sea_orm::{EntityTrait, Set};

use crate::{db::logs, utils::sql::get_db_conn};

pub async fn log_update(owner: String, id: i32, tip: String, from: String, to: String) {
    let db = get_db_conn().await;
    let amodel = logs::ActiveModel {
        owner: Set(owner),
        item_id: Set(id),
        action: Set(format!("UPDATE FROM {} TO {}", from, to)),
        ..Default::default()
    };
    logs::Entity::insert(amodel)
        .exec(&db)
        .await
        .expect("[ERROR] Log létrehozása sikertelen");
}
