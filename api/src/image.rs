use axum::{body::Body, debug_handler, extract::Query, response::Response};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    db::items as Items,
    utils::{
        queries::{BaseImgLeintQuery, BaseImgQuery},
        sql::get_db_conn,
    },
};

#[debug_handler]
pub async fn base_image_get(cucc: Query<BaseImgQuery>) -> Response {
    let db = get_db_conn().await;
    let kep = Items::Entity::find()
        .filter(Items::Column::Id.eq(cucc.id.clone()))
        .filter(Items::Column::Type.ne("leintés"))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let mut kep_file = File::open(format!("./public/{}", fel_kep.image))
            .await
            .expect("[ERROR] Nem létező fájl megnyitása sikertelen");
        let mut contents = vec![];
        let _ = kep_file.read_to_end(&mut contents).await;
        let body = Body::from(contents);
        Response::builder()
            .header(
                "Content-Type",
                format!(
                    "image/{}",
                    if fel_kep.image.starts_with("tmp/") {
                        "png"
                    } else {
                        "avif"
                    }
                ),
            )
            .body(body)
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}

#[debug_handler]
pub async fn base_leintes_image_get(cucc: Query<BaseImgLeintQuery>) -> Response {
    let db = get_db_conn().await;
    let kep = Items::Entity::find()
        .filter(Items::Column::Id.eq(cucc.id.clone()))
        .filter(Items::Column::Type.eq("leintés"))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let arraj = fel_kep.image.split_once(",").unwrap();
        let mut first_elem = arraj.0.to_string();
        first_elem.pop();
        first_elem = first_elem.chars().skip(2).collect();
        let mut second_elem = arraj.1.to_string();
        second_elem.pop();
        second_elem.pop();
        second_elem = second_elem.chars().skip(1).collect();
        let actual = if cucc.ver == String::from("0") {
            first_elem
        } else {
            second_elem
        };
        let mut kep_file = File::open(format!("./public/{}", actual)).await.unwrap();
        let mut contents = vec![];
        let _ = kep_file.read_to_end(&mut contents).await;
        let body = Body::from(contents);
        Response::builder()
            .header(
                "Content-Type",
                format!(
                    "image/{}",
                    if actual.starts_with("tmp/") {
                        "png"
                    } else {
                        "avif"
                    }
                ),
            )
            .body(body)
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
