use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::{
    db::items,
    utils::{
        functions::get_fridays,
        queries::BaseListQuery,
        sql::get_db_conn,
        types_statuses::{get_statuses, get_types},
    },
};

#[debug_handler]
pub async fn base_list_get(
    quer: Query<BaseListQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let friday = get_fridays();
    let db = get_db_conn().await;
    let statuses = get_statuses();
    let types = get_types();
    if quer.tipus.starts_with("potlek") {
        let cuccok = items::Entity::find()
            .filter(items::Column::Owner.eq(quer.driver.clone()))
            .filter(items::Column::Date.gt(friday.laster))
            .filter(items::Column::Status.eq(statuses.accepted.id))
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
            .filter(items::Column::Type.eq(types.supplements.id))
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
            .filter(items::Column::Status.eq(statuses.accepted.id))
            .filter(items::Column::Type.eq(if quer.tipus == "leintes" {
                types.hails.id
            } else if quer.tipus == "szamla" {
                types.bills.id
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
