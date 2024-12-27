use axum::{
    debug_handler,
    extract::{Query, Request},
    response::IntoResponse,
    Extension, Json,
};
use chrono::NaiveDateTime;
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    db::{bills, hails, supplements},
    utils::{
        functions::get_fridays, middle::Tag, queries::MVStatQuery, sql::get_db_conn,
        types_statuses::get_statuses,
    },
};

#[debug_handler]
pub async fn mv_home(mut request: Request) -> Json<Tag> {
    let exts: Option<&Tag> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Serialize)]
struct Date {
    next: NaiveDateTime,
    prev: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    potlekok: Vec<supplements::Model>,
    leintesek: Vec<hails::Model>,
    szamlak: Vec<bills::Model>,
}

#[derive(Debug, Serialize)]
pub struct StatReturn {
    stats: StatDBAll,
    date: Date,
}

#[debug_handler]
pub async fn mv_stat(
    ext: Extension<Tag>,
    quer: Query<MVStatQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let statuses = get_statuses();
    if quer.week == "current".to_string() {
        let friday = get_fridays();
        let db = get_db_conn().await;
        let statreturn_supp = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(supplements::Column::Date.gt(friday.last))
            .filter(supplements::Column::Date.lt(friday.next))
            .filter(supplements::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_hails = hails::Entity::find()
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .filter(hails::Column::Date.gt(friday.last))
            .filter(hails::Column::Date.lt(friday.next))
            .filter(hails::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_bills = bills::Entity::find()
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .filter(bills::Column::Date.gt(friday.last))
            .filter(bills::Column::Date.lt(friday.next))
            .filter(bills::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn_supp.iter() {
            potlekok.push(item.clone())
        }
        for item in statreturn_hails.iter() {
            leintesek.push(item.clone())
        }
        for item in statreturn_bills.iter() {
            szamlak.push(item.clone())
        }
        Ok(Json(StatReturn {
            stats: StatDBAll {
                potlekok,
                leintesek,
                szamlak,
            },
            date: Date {
                next: friday.next,
                prev: friday.last,
            },
        }))
    } else if quer.week == "previous" {
        let friday = get_fridays();
        let db = get_db_conn().await;
        let statreturn_supp = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(supplements::Column::Date.gt(friday.laster))
            .filter(supplements::Column::Date.lt(friday.last))
            .filter(supplements::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_hails = hails::Entity::find()
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .filter(hails::Column::Date.gt(friday.laster))
            .filter(hails::Column::Date.lt(friday.last))
            .filter(hails::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_bills = bills::Entity::find()
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .filter(bills::Column::Date.gt(friday.laster))
            .filter(bills::Column::Date.lt(friday.last))
            .filter(bills::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn_supp.iter() {
            potlekok.push(item.clone())
        }
        for item in statreturn_hails.iter() {
            leintesek.push(item.clone())
        }
        for item in statreturn_bills.iter() {
            szamlak.push(item.clone())
        }
        Ok(Json(StatReturn {
            stats: StatDBAll {
                potlekok,
                leintesek,
                szamlak,
            },
            date: Date {
                next: friday.last,
                prev: friday.laster,
            },
        }))
    } else {
        return Err((StatusCode::BAD_REQUEST, "noweek".to_string()));
    }
}
