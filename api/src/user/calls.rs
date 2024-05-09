use axum::{debug_handler, extract::Request, Json};

use crate::cucc::{api::get_api_envs, middle::Tag};

#[debug_handler]
pub async fn calls(mut request: Request) -> Json<Tag> {
    let client = reqwest::Client::new();
    let envs = get_api_envs();
    let calls = client
        .get(format!("{}/api/log/status/currrent", envs.erik))
        .send()
        .await
        .expect("Lekérés sikertelen")
        .text()
        .await
        .expect("Átalakítás sikertelen");
}
