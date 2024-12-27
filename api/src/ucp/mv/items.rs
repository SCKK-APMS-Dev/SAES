use axum::{
    body::Body,
    debug_handler,
    extract::{self, Query},
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::Utc;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};

use crate::{
    db::{bills, hails, supplements},
    logging::db_log,
    utils::{
        middle::Tag,
        queries::MVItemsQuery,
        sql::get_db_conn,
        types_statuses::{get_statuses_as_list, get_types},
    },
};

#[derive(Debug, Deserialize)]
pub struct MVPostItemsBody {
    pub id: i32,
    pub status: i8,
    pub extra: Option<String>,
    pub reason: Option<String>,
    pub am: i8,
}

#[derive(Debug, Serialize)]
pub struct MVGetItemsFull {
    pub id: i32,
    pub owner: String,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub status: i8,
    pub reason: Option<String>,
    pub r#type: Option<i8>,
    pub price: Option<i32>,
    pub am: i8,
    pub handled_by: Option<String>,
    pub date: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    pub items: Vec<MVGetItemsFull>,
}

#[debug_handler]
pub async fn mv_items_get(
    ext: Extension<Tag>,
    quer: Query<MVItemsQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_db_conn().await;
    let types = get_types();
    if quer.tipus == types.supplements.id {
        let statreturn = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(quer.status.clone()))
            .order_by(supplements::Column::Date, Order::Desc)
            .filter(supplements::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<MVGetItemsFull> = statreturn
            .iter()
            .map(|item| -> MVGetItemsFull {
                MVGetItemsFull {
                    id: item.id,
                    img_1: item.image,
                    img_2: None,
                    am: item.am,
                    status: item.status,
                    date: item.date,
                    price: None,
                    r#type: item.r#type,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    owner: item.owner.clone(),
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else if quer.tipus == types.hails.id {
        let statreturn = hails::Entity::find()
            .filter(hails::Column::Status.eq(quer.status.clone()))
            .order_by(hails::Column::Date, Order::Desc)
            .filter(hails::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<MVGetItemsFull> = statreturn
            .iter()
            .map(|item| -> MVGetItemsFull {
                MVGetItemsFull {
                    id: item.id,
                    img_1: item.image_1,
                    img_2: Some(item.image_2),
                    am: item.am,
                    status: item.status,
                    date: item.date,
                    price: None,
                    r#type: None,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    owner: item.owner.clone(),
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else if quer.tipus == types.bills.id {
        let statreturn = bills::Entity::find()
            .filter(bills::Column::Status.eq(quer.status.clone()))
            .order_by(bills::Column::Date, Order::Desc)
            .filter(bills::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<MVGetItemsFull> = statreturn
            .iter()
            .map(|item| -> MVGetItemsFull {
                MVGetItemsFull {
                    id: item.id,
                    img_1: item.image,
                    img_2: None,
                    am: item.am,
                    price: item.price,
                    r#type: None,
                    status: item.status,
                    date: item.date,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    owner: item.owner.clone(),
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "Ilyen típus nem található!".to_string(),
        ));
    }
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
