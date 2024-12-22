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
    db::items::{self, Model},
    utils::{
        db_bindgen::{get_item_status_int, get_item_type_int},
        functions::get_fridays,
        middle::Tag,
        queries::MVStatQuery,
        sql::get_db_conn,
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
pub async fn mv_stat(
    ext: Extension<Tag>,
    quer: Query<MVStatQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let potlek_num = get_item_type_int("pótlék".to_string()).await.unwrap();
    let leintes_num = get_item_type_int("leintés".to_string()).await.unwrap();
    let szamla_num = get_item_type_int("számla".to_string()).await.unwrap();
    let elfogadva_num = get_item_status_int("elfogadva".to_string()).await.unwrap();
    if quer.week == "current".to_string() {
        let friday = get_fridays();
        let db = get_db_conn().await;
        let statreturn = items::Entity::find()
            .filter(items::Column::Status.eq(elfogadva_num))
            .filter(items::Column::Date.gt(friday.last))
            .filter(items::Column::Date.lt(friday.next))
            .filter(items::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn.iter() {
            if item.r#type == potlek_num {
                potlekok.push(item.clone())
            }
            if item.r#type == leintes_num {
                leintesek.push(item.clone())
            }
            if item.r#type == szamla_num {
                szamlak.push(item.clone())
            }
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
        let statreturn = items::Entity::find()
            .filter(items::Column::Status.eq(elfogadva_num))
            .filter(items::Column::Date.gt(friday.laster))
            .filter(items::Column::Date.lt(friday.last))
            .filter(items::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
            .all(&db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn.iter() {
            if item.r#type == potlek_num {
                potlekok.push(item.clone())
            }
            if item.r#type == leintes_num {
                leintesek.push(item.clone())
            }
            if item.r#type == szamla_num {
                szamlak.push(item.clone())
            }
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
