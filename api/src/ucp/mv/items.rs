use axum::{
    body::Body,
    debug_handler,
    extract::{self, Query},
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};

use crate::{
    db::items::{self as Items, Model},
    logging::db_log,
    utils::{
        middle::Tag, queries::MVItemsQuery, sql::get_db_conn, types_statuses::get_statuses_as_list,
    },
};

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    items: Vec<Model>,
}

#[derive(Debug, Deserialize)]
pub struct MVPostItemsBody {
    pub id: i32,
    pub status: i32,
    pub extra: Option<String>,
    pub reason: Option<String>,
    pub am: i8,
}

#[debug_handler]
pub async fn mv_items_get(ext: Extension<Tag>, quer: Query<MVItemsQuery>) -> impl IntoResponse {
    let db = get_db_conn().await;
    let statreturn = Items::Entity::find()
        .filter(Items::Column::Status.eq(quer.status.clone()))
        .filter(Items::Column::Type.eq(quer.tipus.clone()))
        .order_by(Items::Column::Date, Order::Desc)
        .filter(Items::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
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
    let status_list = get_statuses_as_list();
    if status_list.contains(&body.status) {
        let db = get_db_conn().await;
        let old_model = Items::Entity::find()
            .filter(Items::Column::Id.eq(body.id))
            .one(&db)
            .await
            .expect("[ERROR] Régi lekérése sikertelen")
            .unwrap();
        let mut act = String::new();
        if old_model.status != body.status {
            act += format!(
                "{}status FROM {} TO {}",
                if act.len() > 0 { ", " } else { "" },
                old_model.status,
                body.status
            )
            .as_str();
        }
        if old_model.reason != body.reason {
            act += format!(
                "{}reason FROM {} TO {}",
                if act.len() > 0 { ", " } else { "" },
                if old_model.reason.is_some() {
                    old_model.reason.unwrap()
                } else {
                    String::from("null")
                },
                if body.reason.is_some() {
                    body.reason.clone().unwrap()
                } else {
                    String::from("null")
                }
            )
            .as_str();
        }
        if old_model.extra != body.extra {
            act += format!(
                "{}extra FROM {} TO {}",
                if act.len() > 0 { ", " } else { "" },
                if old_model.extra.is_some() {
                    old_model.extra.unwrap()
                } else {
                    String::from("null")
                },
                if body.extra.is_some() {
                    body.extra.clone().unwrap()
                } else {
                    String::from("null")
                }
            )
            .as_str();
        }
        db_log(ext.name.clone(), Some(body.id.clone()), "UPDATE", Some(act)).await;
        let activemodel = Items::ActiveModel {
            id: Set(body.id),
            am: Set(body.am),
            status: Set(body.status),
            reason: Set(body.reason),
            extra: Set(body.extra),
            handled_by: Set(Some(ext.name.clone())),
            ..Default::default()
        };
        let statreturn = Items::Entity::update(activemodel)
            .exec(&db)
            .await
            .expect("[ERROR] Módosítás sikertelen");
        Json(Model { ..statreturn }).into_response()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("invalid_type"))
            .unwrap()
            .into_response()
    }
}
