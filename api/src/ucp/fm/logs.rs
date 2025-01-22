use axum::{debug_handler, routing::get, Extension, Json, Router};
use chrono::{DateTime, Utc};
use saes_shared::{db::logs, sql::get_db_conn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::utils::{factions::get_faction_id, middle::Driver};

#[derive(Debug, Serialize)]
pub struct Logs {
    owner: String,
    item_id: Option<i32>,
    item_type: Option<i8>,
    action: String,
    message: Option<String>,
    date: DateTime<Utc>,
}

pub fn get_routes() -> Router {
    Router::new().route("/get", get(fm_get_logs))
}

#[debug_handler]
pub async fn fm_get_logs(ext: Extension<Driver>) -> Json<Vec<Logs>> {
    let db = get_db_conn().await;
    let logs = logs::Entity::find()
        .filter(logs::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
        .all(&db)
        .await
        .unwrap();
    let logs: Vec<Logs> = logs
        .iter()
        .map(|log| -> Logs {
            Logs {
                owner: log.owner.clone(),
                item_id: log.item_id,
                item_type: log.item_type,
                action: log.action.clone(),
                message: log.message.clone(),
                date: log.date.clone(),
            }
        })
        .collect();
    return Json(logs);
}
