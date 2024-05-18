use axum::{debug_handler, extract::Request, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::cucc::{api::get_api_envs, middle::Tag, sql::get_conn};

use crate::db::data as Data;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriverRecord {
    pub driver: String,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Callz {
    pub app: u32,
    pub leintes: usize,
}

#[debug_handler]
pub async fn calls(mut request: Request) -> Json<Callz> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    let client = reqwest::Client::new();
    let db = get_conn().await;

    let envs = get_api_envs();
    let calls = client
        .get(format!("{}/api/log/status/current", envs.erik))
        .send()
        .await
        .expect("Lekérés sikertelen")
        .text()
        .await
        .expect("Átalakítás sikertelen");
    let driver_records: Vec<DriverRecord> = from_str(&calls).expect("Átalakítás nem megyen");
    let leintesek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("leintés"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .all(&db)
        .await
        .expect("Leintések lekérése sikertelen az adatbázisból");

    let rec = driver_records
        .iter()
        .find(|record| record.driver == exts.unwrap().name);
    if rec.is_some() {
        Json(Callz {
            app: rec.unwrap().count,
            leintes: leintesek.len(),
        })
    } else {
        Json(Callz {
            app: 0,
            leintes: leintesek.len(),
        })
    }
}
