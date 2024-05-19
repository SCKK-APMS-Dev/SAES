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
    let input_date = Local::now().date_naive();

    // Determine the current weekday of the input date (0 = Monday, ..., 6 = Sunday)
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

    let leintesek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("leintés"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .filter(Data::Column::Date.gte(last_friday_whole))
        .filter(Data::Column::Date.lte(next_friday_whole))
        .all(&db)
        .await
        .expect("Leintések lekérése sikertelen az adatbázisból");
    let de_potlek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("pótlék"))
        .filter(Data::Column::Extra.eq("délelőtti"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .filter(Data::Column::Date.gte(last_friday_whole))
        .filter(Data::Column::Date.lte(next_friday_whole))
        .all(&db)
        .await
        .expect("Délelőtti pótlék lekérése sikertelen az adatbázisból");
    let du_potlek = Data::Entity::find()
        .filter(Data::Column::Owner.eq(&exts.unwrap().name))
        .filter(Data::Column::Type.eq("pótlék"))
        .filter(Data::Column::Extra.eq("éjszakai"))
        .filter(Data::Column::Status.eq("elfogadva"))
        .filter(Data::Column::Date.gte(last_friday_whole))
        .filter(Data::Column::Date.lte(next_friday_whole))
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
