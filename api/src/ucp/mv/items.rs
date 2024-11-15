use axum::{
    debug_handler,
    extract::{self, Query},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};

use crate::{
    db::data::{self as Data, Model},
    utils::{middle::Tag, queries::MVItemsQuery, sql::get_db_conn},
};

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    items: Vec<Model>,
}

#[derive(Debug, Deserialize)]
pub struct MVPostItemsBody {
    pub id: i32,
    pub status: String,
    pub extra: Option<String>,
    pub reason: Option<String>,
    pub am: i8,
}

#[debug_handler]
pub async fn mv_items_get(ext: Extension<Tag>, quer: Query<MVItemsQuery>) -> impl IntoResponse {
    let db = get_db_conn().await;
    let statreturn = Data::Entity::find()
        .filter(Data::Column::Status.eq(quer.status.clone()))
        .filter(Data::Column::Type.eq(quer.tipus.clone()))
        .order_by(Data::Column::Date, Order::Desc)
        .filter(Data::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    Json(StatDBAll { items: statreturn })
}

#[debug_handler]
pub async fn mv_items_post(
    ext: Extension<Tag>,
    extract::Json(body): extract::Json<MVPostItemsBody>,
) -> impl IntoResponse {
    let db = get_db_conn().await;
    let activemodel = Data::ActiveModel {
        id: Set(body.id),
        am: Set(body.am),
        status: Set(body.status),
        reason: Set(body.reason),
        extra: Set(body.extra),
        admin: Set(Some(ext.name.clone())),
        ..Default::default()
    };
    let statreturn = Data::Entity::update(activemodel)
        .exec(&db)
        .await
        .expect("[ERROR] Módosítás sikertelen");
    Json(Model { ..statreturn })
}
