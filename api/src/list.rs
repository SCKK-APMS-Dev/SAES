use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::{
    db::data,
    utils::{functions::get_fridays, queries::BaseListQuery, sql::get_db_conn},
};

#[debug_handler]
pub async fn base_list_get(
    quer: Query<BaseListQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let friday = get_fridays();
    let db = get_db_conn().await;
    if quer.tipus.starts_with("potlek") {
        let cuccok = data::Entity::find()
            .filter(data::Column::Owner.eq(quer.driver.clone()))
            .filter(data::Column::Date.gt(friday.laster))
            .filter(data::Column::Status.eq("elfogadva"))
            .filter(data::Column::Date.lt(friday.last))
            .filter(data::Column::Extra.eq(if quer.tipus == "potlek_de" {
                "délelőtti"
            } else if quer.tipus == "potlek_ej" {
                "éjszakai"
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Ilyen pótlékot nem ismerek!".to_string(),
                ));
            }))
            .filter(data::Column::Type.eq("pótlék"))
            .order_by(data::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        Ok(Json(cuccok))
    } else {
        let cuccok = data::Entity::find()
            .filter(data::Column::Owner.eq(quer.driver.clone()))
            .filter(data::Column::Date.gt(friday.laster))
            .filter(data::Column::Date.lt(friday.last))
            .filter(data::Column::Status.eq("elfogadva"))
            .filter(data::Column::Type.eq(if quer.tipus == "leintes" {
                "leintés"
            } else if quer.tipus == "szamla" {
                "számla"
            } else {
                return Err((StatusCode::BAD_REQUEST, "Nincs ilyen elem!".to_string()));
            }))
            .order_by(data::Column::Date, Order::Desc)
            .all(&db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        Ok(Json(cuccok))
    }
}
