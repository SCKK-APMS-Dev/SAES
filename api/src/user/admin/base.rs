use axum::{
    debug_handler,
    extract::{Query, Request},
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::utils::{functions::get_fridays, middle::Tag, queries::StatQuery};

#[debug_handler]
pub async fn admin_home(mut request: Request) -> Json<Tag> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Serialize)]
struct Date {
    next: NaiveDateTime,
    prev: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct StatReturn {
    stats: u32,
    date: Date,
}

#[debug_handler]
pub async fn admin_stat(ext: Extension<Tag>, quer: Query<StatQuery>) -> Json<StatReturn> {
    let friday = get_fridays();
    Json(StatReturn {
        stats: 1,
        date: Date {
            next: friday.next,
            prev: friday.prev,
        },
    })
}
