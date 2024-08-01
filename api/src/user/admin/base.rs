use axum::{
    debug_handler,
    extract::{Query, Request},
    Extension, Json,
};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    db::data::{self, Model},
    utils::{functions::get_fridays, middle::Tag, queries::StatQuery, sql::get_conn},
};

#[debug_handler]
pub async fn admin_home(mut request: Request) -> Json<Tag> {
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
    potlekok: Vec<Model>,
    leintesek: Vec<Model>,
    szamlak: Vec<Model>,
}

#[derive(Debug, Serialize)]
pub struct StatReturn {
    stats: StatDBAll,
    date: Date,
}

#[debug_handler]
pub async fn admin_stat(ext: Extension<Tag>, quer: Query<StatQuery>) -> Json<StatReturn> {
    let friday = get_fridays();
    let db = get_conn().await;
    let statreturn = data::Entity::find()
        .filter(data::Column::Status.eq("elfogadva"))
        .filter(data::Column::Date.gt(friday.last))
        .filter(data::Column::Date.lt(friday.next))
        .filter(data::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let mut potlekok = vec![];
    let mut leintesek = vec![];
    let mut szamlak = vec![];
    for item in statreturn.iter() {
        if item.r#type == "pótlék" {
            potlekok.push(item.clone())
        }
        if item.r#type == "leintés" {
            leintesek.push(item.clone())
        }
        if item.r#type == "számla" {
            szamlak.push(item.clone())
        }
    }
    Json(StatReturn {
        stats: StatDBAll {
            potlekok,
            leintesek,
            szamlak,
        },
        date: Date {
            next: friday.next,
            prev: friday.last,
        },
    })
}
