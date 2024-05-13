use axum::{debug_handler, extract::Request, Json};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::cucc::{api::get_api_envs, middle::Tag};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriverRecord {
    pub driver: String,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct callz {
    pub app: u32,
    pub leintes: u32,
}

#[debug_handler]
pub async fn calls(mut request: Request) -> Json<callz> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    let client = reqwest::Client::new();
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

    let rec = driver_records
        .iter()
        .find(|record| record.driver == exts.unwrap().name);
    Json(callz { app: 0, leintes: 0 })
}
