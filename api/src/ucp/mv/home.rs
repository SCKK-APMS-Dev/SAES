use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    db::items,
    utils::{
        db_bindgen::{get_item_status_int, get_item_type_int},
        middle::Tag,
        sql::get_db_conn,
    },
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
    let statreturn = items::Entity::find()
        .filter(items::Column::Am.eq(if ext.am == true { 1 } else { 0 }))
        .all(&db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let mut potlekok = [0, 0, 0];
    let mut leintesek = [0, 0, 0];
    let mut szamlak = [0, 0, 0];
    let potlek_num = get_item_type_int("pótlék".to_string()).await.unwrap();
    let leintes_num = get_item_type_int("leintés".to_string()).await.unwrap();
    let szamla_num = get_item_type_int("számla".to_string()).await.unwrap();
    let elfogadva_num = get_item_status_int("elfogadva".to_string()).await.unwrap();
    let elutasitva_num = get_item_status_int("elutasítva".to_string()).await.unwrap();
    let feltoltve_num = get_item_status_int("feltöltve".to_string()).await.unwrap();
    for item in statreturn.iter() {
        if item.r#type == potlek_num {
            if item.status == elfogadva_num {
                potlekok[1] += 1
            }
            if item.status == elutasitva_num {
                potlekok[2] += 1
            }
            if item.status == feltoltve_num {
                potlekok[0] += 1
            }
        }
        if item.r#type == leintes_num {
            if item.status == elfogadva_num {
                leintesek[1] += 1
            }
            if item.status == elutasitva_num {
                leintesek[2] += 1
            }
            if item.status == feltoltve_num {
                leintesek[0] += 1
            }
        }
        if item.r#type == szamla_num {
            if item.status == elfogadva_num {
                szamlak[1] += 1
            }
            if item.status == elutasitva_num {
                szamlak[2] += 1
            }
            if item.status == feltoltve_num {
                szamlak[0] += 1
            }
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
