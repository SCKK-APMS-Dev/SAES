use axum::{body::Body, debug_handler, extract::Query, response::Response};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    db::data as Data,
    utils::{
        queries::{ImgLeintQuery, ImgQuery},
        sql::get_conn,
    },
};

#[debug_handler]
pub async fn image_get(cucc: Query<ImgQuery>) -> Response {
    let db = get_conn().await;
    let kep = Data::Entity::find()
        .filter(Data::Column::Id.eq(cucc.id.clone()))
        .filter(Data::Column::Type.ne("leintés"))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let mut kep_file = File::open(format!("./public/{}", fel_kep.kep))
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
                    if fel_kep.kep.starts_with("tmp/") {
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
pub async fn leintes_image_get(cucc: Query<ImgLeintQuery>) -> Response {
    let db = get_conn().await;
    let kep = Data::Entity::find()
        .filter(Data::Column::Id.eq(cucc.id.clone()))
        .filter(Data::Column::Type.eq("leintés"))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let arraj = fel_kep.kep.split_once(",").unwrap();
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
