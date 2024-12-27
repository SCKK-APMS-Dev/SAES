use std::{fs::File, io::Write};

use axum::{
    debug_handler,
    extract::{DefaultBodyLimit, Multipart, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};

use crate::{
    db::{bills, hails, images, supplements},
    logging::db_log,
    utils::{
        middle::Tag,
        queries::{UCPTypeExtraQuery, UCPTypeQuery},
        sql::get_db_conn,
        types_statuses::{get_statuses, get_types, get_types_as_list},
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsStruct {
    pub id: i32,
    pub owner: String,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub status: i32,
    pub reason: Option<String>,
    pub am: i8,
    pub handled_by: Option<String>,
    pub date: chrono::DateTime<Utc>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/get", get(ucp_items_get))
        .route("/post", post(ucp_items_post))
        .layer(DefaultBodyLimit::max(64000000))
}

#[debug_handler]
pub async fn ucp_items_get(
    ext: Extension<Tag>,
    cucc: Query<UCPTypeQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_db_conn().await;
    let types = get_types();
    if cucc.tipus == types.supplements.id {
        let items = supplements::Entity::find()
            .filter(supplements::Column::Owner.eq(&ext.name))
            .order_by(supplements::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("Pótlékok lekérése sikertelen az adatbázisból");
        let another: Vec<ItemsStruct> = items
            .iter()
            .map(|strucc| -> ItemsStruct {
                ItemsStruct {
                    am: strucc.am,
                    owner: strucc.owner.clone(),
                    img_1: strucc.image,
                    img_2: None,
                    reason: strucc.reason.clone(),
                    status: strucc.status,
                    date: strucc.date,
                    id: strucc.id,
                    handled_by: strucc.handled_by.clone(),
                }
            })
            .collect();
        return Ok(Json(another));
    } else if cucc.tipus == types.hails.id {
        let items = hails::Entity::find()
            .filter(hails::Column::Owner.eq(&ext.name))
            .order_by(hails::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("Pótlékok lekérése sikertelen az adatbázisból");
        let another: Vec<ItemsStruct> = items
            .iter()
            .map(|strucc| -> ItemsStruct {
                ItemsStruct {
                    am: strucc.am,
                    owner: strucc.owner.clone(),
                    img_1: strucc.image_1,
                    img_2: Some(strucc.image_2),
                    reason: strucc.reason.clone(),
                    status: strucc.status,
                    date: strucc.date,
                    id: strucc.id,
                    handled_by: strucc.handled_by.clone(),
                }
            })
            .collect();
        return Ok(Json(another));
    } else if cucc.tipus == types.bills.id {
        let items = bills::Entity::find()
            .filter(bills::Column::Owner.eq(&ext.name))
            .order_by(bills::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("Pótlékok lekérése sikertelen az adatbázisból");
        let another: Vec<ItemsStruct> = items
            .iter()
            .map(|strucc| -> ItemsStruct {
                ItemsStruct {
                    am: strucc.am,
                    owner: strucc.owner.clone(),
                    img_1: strucc.image,
                    img_2: None,
                    reason: strucc.reason.clone(),
                    status: strucc.status,
                    date: strucc.date,
                    id: strucc.id,
                    handled_by: strucc.handled_by.clone(),
                }
            })
            .collect();
        return Ok(Json(another));
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "Ilyen típus nem található!".to_string(),
        ));
    }
}

#[debug_handler]
pub async fn ucp_items_post(
    ext: Extension<Tag>,
    cucc: Query<UCPTypeExtraQuery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_ids: Vec<i32> = Vec::new();
    let mut files_for_leintes: Vec<String> = Vec::new();
    let dates = cucc.dates.clone();
    let ditas: Vec<&str> = dates.split(",").collect();
    let types = get_types();
    let statuses = get_statuses();
    let types_list = get_types_as_list();
    let mut i = 0;
    if types_list.contains(&cucc.tipus.clone()) {
        while let Some(field) = multipart.next_field().await.unwrap() {
            let field_name = field.name().unwrap().to_string();
            if field_name == "files" {
                let file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await;
                if data.is_ok() {
                    let db = get_db_conn().await;
                    let mut file =
                        File::create(format!("./public/tmp/{}-{}", ext.name, file_name)).unwrap();
                    file.write(&data.unwrap()).unwrap();
                    if cucc.tipus.clone() == types.hails.id {
                        if files_for_leintes.len().eq(&1) {
                            let iten = Items::ActiveModel {
                                am: Set(if ext.am.clone() { 1 } else { 0 }),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.name.clone()),
                                r#type: Set(cucc.tipus.clone()),
                                status: Set(statuses.uploaded.id),
                                image: Set(format!(
                                    "['{}','tmp/{}-{}']",
                                    files_for_leintes[0], ext.name, file_name
                                )),
                                ..Default::default()
                            };
                            let newitem = Items::Entity::insert(iten)
                                .exec(&db)
                                .await
                                .expect("Adatbázisba mentés sikertelen");
                            db_log(
                                ext.name.clone(),
                                Some(newitem.last_insert_id),
                                "CREATE",
                                None,
                            )
                            .await;
                            file_ids.push(newitem.last_insert_id);
                            files_for_leintes.clear();
                        } else {
                            files_for_leintes.push(format!("tmp/{}-{}", ext.name, file_name))
                        }
                    } else {
                        let iten = Items::ActiveModel {
                            am: Set(if ext.am.clone() { 1 } else { 0 }),
                            date: Set(
                                DateTime::from_timestamp_millis(ditas[i].parse().unwrap()).unwrap()
                            ),
                            owner: Set(ext.name.clone()),
                            r#type: Set(cucc.tipus.clone()),
                            status: Set(statuses.uploaded.id),
                            image: Set(format!("tmp/{}-{}", ext.name, file_name)),
                            ..Default::default()
                        };
                        let newitem = Items::Entity::insert(iten)
                            .exec(&db)
                            .await
                            .expect("Adatbázisba mentés sikertelen");
                        db_log(
                            ext.name.clone(),
                            Some(newitem.last_insert_id),
                            "CREATE",
                            None,
                        )
                        .await;
                        file_ids.push(newitem.last_insert_id)
                    }
                    i += 1
                } else {
                    return Err((StatusCode::NOT_ACCEPTABLE, "toobig".to_string()));
                }
            } else {
                let data = field.text().await.unwrap();
                println!("field: {}   value: {}", field_name, data)
            }
        }
        Ok(Json(file_ids))
    } else {
        return Err((StatusCode::NOT_ACCEPTABLE, "invalid_type".to_string()));
    }
}
