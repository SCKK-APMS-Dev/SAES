use axum::{debug_handler, extract::Request, Json};

use crate::cucc::middle::Tag;

#[debug_handler]
pub async fn admin_home(mut request: Request) -> Json<Tag> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}
