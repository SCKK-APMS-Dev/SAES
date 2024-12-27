use axum::{body::Body, debug_handler, extract::Query, response::Response};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    db::images as Images,
    utils::{queries::BaseImgQuery, sql::get_db_conn},
};

#[debug_handler]
pub async fn base_image_get(cucc: Query<BaseImgQuery>) -> Response {
    let db = get_db_conn().await;
    let kep = Images::Entity::find()
        .filter(Images::Column::Id.eq(cucc.id.clone()))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let mut kep_file = if fel_kep.tmp == 0 {
            File::open(format!("./public/{}", fel_kep.filename))
                .await
                .expect("[ERROR] Fájl megnyitása sikertelen")
        } else {
            File::open(format!("./public/tmp/{}", fel_kep.filename))
                .await
                .expect("[ERROR] Fájl megnyitása sikertelen")
        };
        let mut contents = vec![];
        let _ = kep_file.read_to_end(&mut contents).await;
        let body = Body::from(contents);
        Response::builder()
            .header(
                "Content-Type",
                format!(
                    "image/{}",
                    if fel_kep.converted > 0 { "avif" } else { "png" }
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
