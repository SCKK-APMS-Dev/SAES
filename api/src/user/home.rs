use actix_web::{get, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    cucc::sql::get_conn,
    db::data::{self},
};

#[get("/")]
pub async fn user_main() -> impl Responder {
    let db = get_conn().await;
    let qer = data::Entity::find()
        .filter(data::Column::Owner.eq("Ronan Lambton"))
        .all(&db)
        .await
        .unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&qer).unwrap())
}
