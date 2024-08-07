use axum::{debug_handler, extract::Query, response::IntoResponse, Extension, Json};
use http::StatusCode;
use serde::Serialize;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    db::data::{self as Data, Model},
    utils::{middle::Tag, sql::get_conn},
};

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    items: Vec<Model>,
}

#[debug_handler]
pub async fn admin_items(ext: Extension<Tag>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_conn().await;
    let statreturn = Data::Entity::find()
        .filter(Data::Column::Status.eq("feltöltve"))
        .filter(Data::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    Ok(Json(StatDBAll { items: statreturn }))
}
