use axum::{body::Body, debug_handler, extract::Query, response::Response};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    db::images as Images,
    utils::{queries::BaseImgQuery, sql::get_db_conn},
};

#[debug_handler]
pub async fn base_image_get(q: Query<BaseImgQuery>) -> Response {
    let db = get_db_conn().await;
    let image = Images::Entity::find()
        .filter(Images::Column::Id.eq(q.id.clone()))
        .one(&db)
        .await
        .unwrap();
    if image.is_some() {
        let fel_image = image.unwrap();
        let mut image_file = if fel_image.tmp == 0 {
            File::open(format!("./public/{}", fel_image.filename))
                .await
                .expect("[ERROR] Fájl megnyitása sikertelen")
        } else {
            File::open(format!("./public/tmp/{}", fel_image.filename))
                .await
                .expect("[ERROR] Fájl megnyitása sikertelen")
        };
        let mut contents = vec![];
        let _ = image_file.read_to_end(&mut contents).await;
        let body = Body::from(contents);
        Response::builder()
            .header(
                "Content-Type",
                format!(
                    "image/{}",
                    if fel_image.converted > 0 {
                        "avif"
                    } else {
                        "png"
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
