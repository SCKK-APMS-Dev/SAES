use axum::{debug_handler, extract::Request, Json};

use crate::utils::{
    middle::Tag,
    types_statuses::{get_statuses, get_types, Statuses, Types},
};

#[debug_handler]
pub async fn ucp_home(mut request: Request) -> Json<Tag> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[debug_handler]
pub async fn ucp_get_types() -> Json<Types> {
    let types = get_types();
    Json(types)
}
#[debug_handler]
pub async fn ucp_get_statuses() -> Json<Statuses> {
    let statuses = get_statuses();
    Json(statuses)
}
