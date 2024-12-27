use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    db::{bills, hails, supplements},
    utils::{middle::Tag, sql::get_db_conn, types_statuses::get_statuses},
};

#[derive(Debug, Serialize)]
pub struct MVStat {
    feltoltve: i32,
    elfogadva: i32,
    elutasitva: i32,
}
#[derive(Debug, Serialize)]
pub struct MVStatReturn {
    potlek: MVStat,
    leintes: MVStat,
    szamla: MVStat,
}

pub async fn mv_home_stat(ext: Extension<Tag>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = get_db_conn().await;
    let statreturn_supp = supplements::Entity::find()
        .filter(supplements::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let statreturn_hails = hails::Entity::find()
        .filter(hails::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let statreturn_bills = bills::Entity::find()
        .filter(bills::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let mut potlekok = [0, 0, 0];
    let mut leintesek = [0, 0, 0];
    let mut szamlak = [0, 0, 0];
    let statuses = get_statuses();
    for item in statreturn_supp.iter() {
        if item.status == statuses.accepted.id {
            potlekok[1] += 1
        }
        if item.status == statuses.rejected.id {
            potlekok[2] += 1
        }
        if item.status == statuses.uploaded.id {
            potlekok[0] += 1
        }
    }
    for item in statreturn_hails.iter() {
        if item.status == statuses.accepted.id {
            leintesek[1] += 1
        }
        if item.status == statuses.rejected.id {
            leintesek[2] += 1
        }
        if item.status == statuses.uploaded.id {
            leintesek[0] += 1
        }
    }
    for item in statreturn_bills.iter() {
        if item.status == statuses.accepted.id {
            szamlak[1] += 1
        }
        if item.status == statuses.rejected.id {
            szamlak[2] += 1
        }
        if item.status == statuses.uploaded.id {
            szamlak[0] += 1
        }
    }
    Ok(Json(MVStatReturn {
        potlek: MVStat {
            elfogadva: potlekok[1],
            elutasitva: potlekok[2],
            feltoltve: potlekok[0],
        },
        leintes: MVStat {
            elfogadva: leintesek[1],
            elutasitva: leintesek[2],
            feltoltve: leintesek[0],
        },
        szamla: MVStat {
            elfogadva: szamlak[1],
            elutasitva: szamlak[2],
            feltoltve: szamlak[0],
        },
    }))
}
