use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    db::data,
    utils::{middle::Tag, sql::get_conn},
};

#[derive(Debug, Serialize)]
pub struct HomeStat {
    feltoltve: i32,
    elfogadva: i32,
    elutasitva: i32,
}
#[derive(Debug, Serialize)]
pub struct HomeStatReturn {
    potlek: HomeStat,
    leintes: HomeStat,
    szamla: HomeStat,
}

pub async fn admin_home_stat(
    ext: Extension<Tag>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_conn().await;
    let statreturn = data::Entity::find()
        .filter(data::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let mut potlekok = [0, 0, 0];
    let mut leintesek = [0, 0, 0];
    let mut szamlak = [0, 0, 0];
    for item in statreturn.iter() {
        if item.r#type == "pótlék" {
            if item.status == "elfogadva" {
                potlekok[1] += 1
            }
            if item.status == "elutasítva" {
                potlekok[2] += 1
            }
            if item.status == "feltöltve" {
                potlekok[0] += 1
            }
        }
        if item.r#type == "leintés" {
            if item.status == "elfogadva" {
                leintesek[1] += 1
            }
            if item.status == "elutasítva" {
                leintesek[2] += 1
            }
            if item.status == "feltöltve" {
                leintesek[0] += 1
            }
        }
        if item.r#type == "számla" {
            if item.status == "elfogadva" {
                szamlak[1] += 1
            }
            if item.status == "elutasítva" {
                szamlak[2] += 1
            }
            if item.status == "feltöltve" {
                szamlak[0] += 1
            }
        }
    }
    Ok(Json(HomeStatReturn {
        potlek: HomeStat {
            elfogadva: potlekok[1],
            elutasitva: potlekok[2],
            feltoltve: potlekok[0],
        },
        leintes: HomeStat {
            elfogadva: leintesek[1],
            elutasitva: leintesek[2],
            feltoltve: leintesek[0],
        },
        szamla: HomeStat {
            elfogadva: szamlak[1],
            elutasitva: szamlak[2],
            feltoltve: szamlak[0],
        },
    }))
}
