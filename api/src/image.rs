use axum::{body::Body, debug_handler, extract::Query, response::Response};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    cucc::{headers::ImgHeader, sql::get_conn},
    db::data as Data,
};

#[debug_handler]
pub async fn image_get(cucc: Query<ImgHeader>) -> Response {
    let db = get_conn().await;
    let kep = Data::Entity::find()
        .filter(Data::Column::Id.eq(cucc.id.clone()))
        .one(&db)
        .await
        .unwrap();
    if kep.is_some() {
        let fel_kep = kep.unwrap();
        let mut kep_file = File::open(format!("./public/{}", fel_kep.kep))
            .await
            .unwrap();
        let mut contents = vec![];
        let _ = kep_file.read_to_end(&mut contents).await;
        let body = Body::from(contents);
        Response::builder()
            .header("Content-Type", "image/png")
            .body(body)
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
