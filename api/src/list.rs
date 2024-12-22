use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::{
    db::items,
    utils::{
        db_bindgen::{get_item_status_int, get_item_type_int},
        functions::get_fridays,
        queries::BaseListQuery,
        sql::get_db_conn,
    },
};

#[debug_handler]
pub async fn base_list_get(
    quer: Query<BaseListQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let friday = get_fridays();
    let db = get_db_conn().await;
    if quer.tipus.starts_with("potlek") {
        let cuccok = items::Entity::find()
            .filter(items::Column::Owner.eq(quer.driver.clone()))
            .filter(items::Column::Date.gt(friday.laster))
            .filter(
                items::Column::Status
                    .eq(get_item_status_int("elfogadva".to_string()).await.unwrap()),
            )
            .filter(items::Column::Date.lt(friday.last))
            .filter(items::Column::Extra.eq(if quer.tipus == "potlek_de" {
                "délelőtti"
            } else if quer.tipus == "potlek_ej" {
                "éjszakai"
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Ilyen pótlékot nem ismerek!".to_string(),
                ));
            }))
            .filter(items::Column::Type.eq(get_item_type_int("pótlék".to_string()).await.unwrap()))
            .order_by(items::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        Ok(Json(cuccok))
    } else {
        let cuccok = items::Entity::find()
            .filter(items::Column::Owner.eq(quer.driver.clone()))
            .filter(items::Column::Date.gt(friday.laster))
            .filter(items::Column::Date.lt(friday.last))
            .filter(
                items::Column::Status
                    .eq(get_item_status_int("elfogadva".to_string()).await.unwrap()),
            )
            .filter(items::Column::Type.eq(if quer.tipus == "leintes" {
                get_item_type_int("leintés".to_string()).await.unwrap()
            } else if quer.tipus == "szamla" {
                get_item_type_int("számla".to_string()).await.unwrap()
            } else {
                return Err((StatusCode::BAD_REQUEST, "Nincs ilyen elem!".to_string()));
            }))
            .order_by(items::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        Ok(Json(cuccok))
    }
}
