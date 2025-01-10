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
use saes_shared::{
    db::{bills, hails, images, supplements},
    sql::get_db_conn,
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};

use crate::{
    logging::db_log,
    utils::{
        factions::{get_faction_id, Factions},
        middle::Driver,
        queries::{UCPTypeExtraQuery, UCPTypeQuery},
        types_statuses::{get_statuses, get_types, get_types_as_list},
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsStruct {
    pub id: i32,
    pub owner: String,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub status: i8,
    pub reason: Option<String>,
    pub faction: Factions,
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
    ext: Extension<Driver>,
    cucc: Query<UCPTypeQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_db_conn().await;
    let types = get_types();
    if ext.faction.is_some() {
        if cucc.tipus == types.supplements.id {
            let items = supplements::Entity::find()
                .filter(supplements::Column::Owner.eq(&ext.name))
                .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(supplements::Column::Date, Order::Desc)
                .all(&db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        owner: strucc.owner.clone(),
                        img_1: strucc.image,
                        faction: ext.faction.unwrap(),
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
                .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(hails::Column::Date, Order::Desc)
                .all(&db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        faction: ext.faction.unwrap(),
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
                .filter(bills::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(bills::Column::Date, Order::Desc)
                .all(&db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        faction: ext.faction.unwrap(),
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
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}

#[debug_handler]
pub async fn ucp_items_post(
    ext: Extension<Driver>,
    cucc: Query<UCPTypeExtraQuery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_ids: Vec<[i32; 2]> = Vec::new();
    let mut files_for_leintes: Vec<i32> = vec![];
    let dates = cucc.dates.clone();
    let ditas: Vec<&str> = dates.split(",").collect();
    let types = get_types();
    let statuses = get_statuses();
    let types_list = get_types_as_list();
    let mut i = 0;
    if ext.faction.is_some() {
        if types_list.contains(&cucc.tipus.clone()) {
            while let Some(field) = multipart.next_field().await.unwrap() {
                let field_name = field.name().unwrap().to_string();
                if field_name == "files" {
                    let file_name = field.file_name().unwrap().to_string();
                    let data = field.bytes().await;
                    if data.is_ok() {
                        let db = get_db_conn().await;
                        let mut file =
                            File::create(format!("./public/tmp/{}-{}", ext.name, file_name))
                                .unwrap();
                        file.write(&data.unwrap()).unwrap();
                        if cucc.tipus == types.hails.id {
                            if files_for_leintes.len().eq(&1) {
                                let img = images::ActiveModel {
                                    owner: Set(ext.name.clone()),
                                    tmp: Set(1),
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    filename: Set(format!("{}-{}", ext.name, file_name)),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    usage: Set(types.hails.id),
                                    ..Default::default()
                                };
                                let new_img = images::Entity::insert(img)
                                    .exec(&db)
                                    .await
                                    .expect("Fájl mentése sikertelen");
                                let iten = hails::ActiveModel {
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    owner: Set(ext.name.clone()),
                                    status: Set(statuses.uploaded.id),
                                    image_1: Set(files_for_leintes[0]),
                                    image_2: Set(new_img.last_insert_id),
                                    ..Default::default()
                                };
                                let newitem = hails::Entity::insert(iten)
                                    .exec(&db)
                                    .await
                                    .expect("Adatbázisba mentés sikertelen");
                                db_log(
                                    ext.name.clone(),
                                    Some(newitem.last_insert_id),
                                    Some(types.hails.id),
                                    "CREATE",
                                    None,
                                )
                                .await;
                                file_ids.push([new_img.last_insert_id, files_for_leintes[0]]);
                                files_for_leintes.clear();
                            } else {
                                let img = images::ActiveModel {
                                    owner: Set(ext.name.clone()),
                                    filename: Set(format!("{}-{}", ext.name, file_name)),
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    tmp: Set(1),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    usage: Set(types.hails.id),
                                    ..Default::default()
                                };
                                let new_img = images::Entity::insert(img)
                                    .exec(&db)
                                    .await
                                    .expect("Fájl mentése sikertelen");
                                files_for_leintes.push(new_img.last_insert_id)
                            }
                        } else if cucc.tipus == types.supplements.id {
                            let img = images::ActiveModel {
                                owner: Set(ext.name.clone()),
                                tmp: Set(1),
                                filename: Set(format!("{}-{}", ext.name, file_name)),
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                usage: Set(types.supplements.id),
                                ..Default::default()
                            };
                            let new_img = images::Entity::insert(img)
                                .exec(&db)
                                .await
                                .expect("Fájl mentése sikertelen");
                            let iten = supplements::ActiveModel {
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.name.clone()),
                                status: Set(statuses.uploaded.id),
                                image: Set(new_img.last_insert_id),
                                ..Default::default()
                            };
                            let newitem = supplements::Entity::insert(iten)
                                .exec(&db)
                                .await
                                .expect("Adatbázisba mentés sikertelen");
                            db_log(
                                ext.name.clone(),
                                Some(newitem.last_insert_id),
                                Some(types.supplements.id),
                                "CREATE",
                                None,
                            )
                            .await;
                            file_ids.push([new_img.last_insert_id, 0])
                        } else if cucc.tipus == types.bills.id {
                            let img = images::ActiveModel {
                                owner: Set(ext.name.clone()),
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                tmp: Set(1),
                                filename: Set(format!("{}-{}", ext.name, file_name)),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                usage: Set(types.bills.id),
                                ..Default::default()
                            };
                            let new_img = images::Entity::insert(img)
                                .exec(&db)
                                .await
                                .expect("Fájl mentése sikertelen");
                            let iten = bills::ActiveModel {
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.name.clone()),
                                status: Set(statuses.uploaded.id),
                                image: Set(new_img.last_insert_id),
                                ..Default::default()
                            };
                            let newitem = bills::Entity::insert(iten)
                                .exec(&db)
                                .await
                                .expect("Adatbázisba mentés sikertelen");
                            db_log(
                                ext.name.clone(),
                                Some(newitem.last_insert_id),
                                Some(types.bills.id),
                                "CREATE",
                                None,
                            )
                            .await;
                            file_ids.push([new_img.last_insert_id, 0])
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
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}
