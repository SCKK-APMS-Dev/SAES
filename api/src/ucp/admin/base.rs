use axum::{debug_handler, extract::Request, Json};

use crate::utils::middle::Driver;

#[debug_handler]
pub async fn admin_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}
