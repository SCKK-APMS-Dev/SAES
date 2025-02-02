use std::env;

use axum::{body::Body, debug_handler, extract::Query, http::StatusCode, response::Response};
use saes_shared::db::images;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt};

use crate::DB_CLIENT;

#[derive(Debug, Deserialize)]
pub struct ImgQuery {
    pub id: i32,
}

#[debug_handler]
pub async fn image_get(q: Query<ImgQuery>) -> Response {
    let db = DB_CLIENT.get().unwrap();
    let parent_dir = env::var("PARENT_IMAGE_DIR").expect("PARENT_IMAGE_DIR env nem létezik");
    let image = images::Entity::find()
        .filter(images::Column::Id.eq(q.id.clone()))
        .one(db)
        .await
        .unwrap();
    if image.is_some() {
        let fel_image = image.unwrap();
        let mut image_file = if fel_image.tmp == 0 {
            File::open(format!("{}/{}", parent_dir, fel_image.filename))
                .await
                .expect("[ERROR] Fájl megnyitása sikertelen")
        } else {
            File::open(format!("{}/tmp/{}", parent_dir, fel_image.filename))
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
                    if fel_image.filename.ends_with(".avif") {
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
