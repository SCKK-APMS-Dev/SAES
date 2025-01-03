use axum::{
    debug_handler,
    extract::{self, Query},
    response::IntoResponse,
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
        middle::Driver,
        queries::SMItemsQuery,
        sql::get_db_conn,
        types_statuses::{get_statuses_as_list, get_types, get_types_as_list},
    },
};

#[derive(Debug, Deserialize)]
pub struct SMPostItemsBody {
    pub id: i32,
    pub status: i8,
    pub price: Option<i32>,
    pub supp_type: Option<i8>,
    pub reason: Option<String>,
    pub tipus: i8,
    pub am: i8,
}

#[derive(Debug, Serialize)]
pub struct SMGetItemsFull {
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
    pub items: Vec<SMGetItemsFull>,
}

#[debug_handler]
pub async fn sm_items_get(
    ext: Extension<Driver>,
    quer: Query<SMItemsQuery>,
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
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
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
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
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
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
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
pub async fn sm_items_post(
    ext: Extension<Driver>,
    extract::Json(body): extract::Json<SMPostItemsBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let status_list = get_statuses_as_list();
    let types_list = get_types_as_list();
    if status_list.contains(&body.status) && types_list.contains(&body.tipus) {
        let types = get_types();
        let db = get_db_conn().await;
        if body.tipus == types.supplements.id {
            let old_model = supplements::Entity::find()
                .filter(supplements::Column::Id.eq(body.id))
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
            if old_model.r#type != body.supp_type {
                act += format!(
                    "{}supp_type FROM {} TO {}",
                    if act.len() > 0 { ", " } else { "" },
                    if old_model.r#type.is_some() {
                        old_model.r#type.unwrap()
                    } else {
                        0
                    },
                    if body.supp_type.is_some() {
                        body.supp_type.clone().unwrap()
                    } else {
                        0
                    }
                )
                .as_str();
            }
            db_log(
                ext.name.clone(),
                Some(body.id.clone()),
                Some(types.supplements.id),
                "UPDATE",
                Some(act),
            )
            .await;
            let activemodel = supplements::ActiveModel {
                id: Set(body.id),
                am: Set(body.am),
                status: Set(body.status),
                reason: Set(body.reason),
                r#type: Set(if !body.supp_type.is_some() {
                    None
                } else if vec![1, 2].contains(&body.supp_type.unwrap()) {
                    body.supp_type
                } else {
                    None
                }),
                handled_by: Set(Some(ext.name.clone())),
                ..Default::default()
            };
            let statreturn = supplements::Entity::update(activemodel)
                .exec(&db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                am: statreturn.am,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image,
                img_2: None,
                owner: statreturn.owner,
                price: None,
                r#type: statreturn.r#type,
            })
            .into_response())
        } else if body.tipus == types.hails.id {
            let old_model = hails::Entity::find()
                .filter(hails::Column::Id.eq(body.id))
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
            db_log(
                ext.name.clone(),
                Some(body.id.clone()),
                Some(types.hails.id),
                "UPDATE",
                Some(act),
            )
            .await;
            let activemodel = hails::ActiveModel {
                id: Set(body.id),
                am: Set(body.am),
                status: Set(body.status),
                reason: Set(body.reason),
                handled_by: Set(Some(ext.name.clone())),
                ..Default::default()
            };
            let statreturn = hails::Entity::update(activemodel)
                .exec(&db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                am: statreturn.am,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image_1,
                img_2: Some(statreturn.image_2),
                owner: statreturn.owner,
                r#type: None,
                price: None,
            })
            .into_response())
        } else if body.tipus == types.bills.id {
            let old_model = bills::Entity::find()
                .filter(bills::Column::Id.eq(body.id))
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
            if old_model.price != body.price {
                act += format!(
                    "{}price FROM {} TO {}",
                    if act.len() > 0 { ", " } else { "" },
                    if old_model.price.is_some() {
                        old_model.price.unwrap()
                    } else {
                        0
                    },
                    if body.price.is_some() {
                        body.price.clone().unwrap()
                    } else {
                        0
                    }
                )
                .as_str();
            }
            db_log(
                ext.name.clone(),
                Some(body.id.clone()),
                Some(types.bills.id),
                "UPDATE",
                Some(act),
            )
            .await;
            let activemodel = bills::ActiveModel {
                id: Set(body.id),
                am: Set(body.am),
                status: Set(body.status),
                reason: Set(body.reason),
                price: Set(body.price),
                handled_by: Set(Some(ext.name.clone())),
                ..Default::default()
            };
            let statreturn = bills::Entity::update(activemodel)
                .exec(&db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                am: statreturn.am,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image,
                img_2: None,
                owner: statreturn.owner,
                price: statreturn.price,
                r#type: None,
            })
            .into_response())
        } else {
            Err((StatusCode::BAD_REQUEST, "Érvénytelen típus".to_string()))
        }
    } else {
        Err((StatusCode::NOT_FOUND, "Ejnye!".to_string()))
    }
}
