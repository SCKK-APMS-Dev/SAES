use axum::{debug_handler, extract::Query, response::IntoResponse, Extension, Json};
use serde::Serialize;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    db::data::{self as Data, Model},
    utils::{middle::Tag, queries::AdminItemsQuery, sql::get_conn},
};

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    items: Vec<Model>,
}

#[debug_handler]
pub async fn admin_items_get(
    ext: Extension<Tag>,
    quer: Query<AdminItemsQuery>,
) -> impl IntoResponse {
    let db = get_conn().await;
    let statreturn = Data::Entity::find()
        .filter(Data::Column::Status.eq(quer.status.clone()))
        .filter(Data::Column::Type.eq(quer.tipus.clone()))
        .filter(Data::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    Json(StatDBAll { items: statreturn })
}
