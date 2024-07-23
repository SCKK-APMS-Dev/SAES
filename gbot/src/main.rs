use std::{env, time::Duration};

use google_sheets4::{api::ValueRange, hyper, hyper_rustls, Sheets};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::interval;

mod auth;

#[derive(Debug, Deserialize, PartialEq)]
struct DriverData {
    driver: String,
    count: u32,
}

async fn get_drivers() -> Vec<DriverData> {
    let calls_url = env::var("CALLS_URL").expect("CALLS_URL lekérése sikertelen");
    let res = reqwest::get(calls_url)
        .await
        .expect("Hívás lekérés sikertelen");
    let json_data = res.text().await.unwrap();
    let drivers: Vec<DriverData> = serde_json::from_str(&json_data).expect("Átalakítás sikertelen");
    drivers
}
#[tokio::main]
async fn main() {
    let mut interval = interval(Duration::from_secs(120));
    loop {
        interval.tick().await; // This should go first.
        println!("");
        println!("==== Taxi A műszak ====");
        println!("");
        handle_tables("Taxi A műszak", "A2:A21", "B2").await;
        println!("");
        println!("==== Taxi B műszak ====");
        println!("");
        handle_tables("Taxi B műszak", "A2:A21", "B2").await;
        println!("");
        println!("==== Taxi C műszak ====");
        println!("");
        handle_tables("Taxi C műszak", "A2:A21", "B2").await;
        println!("");
        println!("=======================");
        println!("");
    }
}
async fn handle_tables(table: &str, read_range: &str, write_range: &str) {
    let spread_id = env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID lekérése sikertelen");
    let token = auth::get_google_auth().await;
    let calls = get_drivers().await;
    let sheets = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        token,
    );
    let res = sheets
        .spreadsheets()
        .values_get(&spread_id, format!("{}!{}", table, read_range).as_str())
        .doit()
        .await
        .expect("Táblázat lekérés sikertelen.");
    let values: Vec<Vec<serde_json::value::Value>> = res.1.values.unwrap_or_default();
    let mut req = ValueRange::default();
    let mut vals: Vec<Vec<Value>> = vec![];
    for tag in values.iter() {
        if let Some(call) = calls.iter().find(|x| x.driver == tag[0]) {
            println!("{}: {}", tag[0], call.count);
            vals.push(vec![serde_json::Value::String(call.count.to_string())])
        } else {
            vals.push(vec![serde_json::Value::String(0.to_string())])
        }
    }
    req.values = vals.into();
    sheets
        .spreadsheets()
        .values_update(
            req,
            &spread_id,
            format!("{}!{}", table, write_range).as_str(),
        )
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .expect("Táblázat írás sikertelen");
}
