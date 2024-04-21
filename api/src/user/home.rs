use actix_web::{get, HttpResponse, Responder};
use sea_orm::EntityTrait;

use crate::{
    cucc::sql::get_conn,
    db::data::{self},
};

#[get("/user")]
pub async fn user_main() -> impl Responder {
    let db = get_conn().await;
    let qer = data::Entity::find().all(&db).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&qer).unwrap())
}
