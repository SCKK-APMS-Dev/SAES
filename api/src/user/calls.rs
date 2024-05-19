use std::time::Duration;

use axum::{debug_handler, extract::Request, Json};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
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
    pub potlek: Potlek,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Potlek {
    de: usize,
    du: usize,
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
    let tegnap_ev = NaiveDate::from_ymd_opt(2024, 3, 12);
    let tegnap_ora = NaiveTime::from_hms_opt(22, 0, 0);
    let real_tegnap = NaiveDateTime::new(tegnap_ev.unwrap(), tegnap_ora.unwrap());
    let utc_tegnap = Utc::from_utc_datetime(&Utc, &real_tegnap);
    let leintesek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("leintés"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .all(&db)
        .await
        .expect("Leintések lekérése sikertelen az adatbázisból");
    let de_potlek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("pótlék"))
        .filter(Data::Column::Extra.eq("délelőtti"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .all(&db)
        .await
        .expect("Délelőtti pótlék lekérése sikertelen az adatbázisból");
    let du_potlek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("pótlék"))
        .filter(Data::Column::Extra.eq("éjszakai"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .all(&db)
        .await
        .expect("Éjszakai pótlék lekérése sikertelen az adatbázisból");

    let rec = driver_records
        .iter()
        .find(|record| record.driver == exts.unwrap().name);
    Json(Callz {
        app: if rec.is_some() { rec.unwrap().count } else { 0 },
        leintes: leintesek.len(),
        potlek: Potlek {
            de: de_potlek.len(),
            du: du_potlek.len(),
        },
    })
}
