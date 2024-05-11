use axum::{debug_handler, extract::Request, Json};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::cucc::{api::get_api_envs, middle::Tag};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriverRecord {
    pub driver: String,
    pub count: u32,
}

#[debug_handler]
pub async fn calls(mut request: Request) -> Json<DriverRecord> {
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

    Json(
        driver_records
            .iter()
            .find(|record| record.driver == "Lemondott")
            .unwrap()
            .clone(),
    )
}
