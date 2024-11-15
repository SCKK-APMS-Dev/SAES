use axum::{debug_handler, extract::Request, Json};

use crate::utils::middle::Tag;

#[debug_handler]
pub async fn ucp_home(mut request: Request) -> Json<Tag> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}
