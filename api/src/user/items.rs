use axum::{
    debug_handler,
    extract::{Query, Request},
    http::HeaderMap,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    cucc::{middle::Tag, sql::get_conn},
    db::data::{self as Data},
};

use super::calls::get_fridays;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Items {
    pub id: i32,
    pub owner: String,
    pub kep: String,
    pub status: String,
    pub reason: Option<String>,
    pub am: i8,
    pub date: chrono::DateTime<Utc>,
}

pub fn routes() -> Router {
    Router::new().route("/get", get(items_get))
}

#[derive(Debug, Deserialize)]
struct Header {
    tipus: String,
}

#[debug_handler]
pub async fn items_get(cucc: Query<Header>, request: Request) -> Json<Vec<Items>> {
    let exts: Option<&Tag> = request.extensions().get();
    let fridays = get_fridays();
    let db = get_conn().await;
    let getitem = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq(cucc.tipus.clone()))
        .filter(Data::Column::Date.gte(fridays.prev))
        .filter(Data::Column::Date.lte(fridays.next))
        .all(&db)
        .await
        .expect("Leintések lekérése sikertelen az adatbázisból");
    let another: Vec<Items> = getitem
        .iter()
        .map(|strucc| -> Items {
            Items {
                am: strucc.am.clone(),
                owner: strucc.owner.clone(),
                kep: strucc.kep.clone(),
                reason: strucc.reason.clone(),
                status: strucc.status.clone(),
                date: strucc.date.clone(),
                id: strucc.id.clone(),
            }
        })
        .collect();
    Json(another)
}
