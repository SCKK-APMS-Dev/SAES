use axum::{debug_handler, extract::Request, Json};
use chrono::{Datelike, Duration, Local, NaiveDateTime, NaiveTime};
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

pub struct Friday {
    pub prev: NaiveDateTime,
    pub next: NaiveDateTime,
}

pub fn get_fridays() -> Friday {
    let input_date = Local::now().date_naive();
    let current_weekday = input_date.weekday().num_days_from_monday();
    let zaras = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    // Calculate the number of days until the next Friday (5th day of the week)
    let days_until_friday = (4 + 7 - current_weekday) % 7;
    // If the input date is already Friday, we need to move to the next week's Friday
    let days_until_friday = if current_weekday == 4 {
        0
    } else {
        days_until_friday
    };

    // Get the next Friday by adding the days until Friday to the input date
    let next_friday = input_date + Duration::days(days_until_friday.into());
    let last_friday = next_friday + Duration::days(-7);
    let next_friday_whole = NaiveDateTime::new(next_friday, zaras);
    let last_friday_whole = NaiveDateTime::new(last_friday, zaras);
    Friday {
        prev: last_friday_whole,
        next: next_friday_whole,
    }
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
    let fridays = get_fridays();
    let dbreturn = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.ne("számla"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .filter(Data::Column::Date.gte(fridays.prev))
        .filter(Data::Column::Date.lte(fridays.next))
        .all(&db)
        .await
        .expect("Leintések lekérése sikertelen az adatbázisból");
    let mut leintes = vec![];
    let mut de_potlek = vec![];
    let mut du_potlek = vec![];
    for model in dbreturn.iter() {
        if model.r#type == "pótlék" {
            if model.extra == "délelőtti".to_string().into() {
                de_potlek.push(model)
            }
            if model.extra == "éjszakai".to_string().into() {
                du_potlek.push(model)
            }
        }
        if model.r#type == "leintés" {
            leintes.push(model)
        }
    }
    let rec: Option<&DriverRecord> = driver_records
        .iter()
        .find(|record| record.driver == exts.unwrap().name);
    Json(Callz {
        app: if rec.is_some() { rec.unwrap().count } else { 0 },
        leintes: leintes.len(),
        potlek: Potlek {
            de: de_potlek.len(),
            du: du_potlek.len(),
        },
    })
}
