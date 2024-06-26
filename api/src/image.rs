use axum::{
    body::Body,
    debug_handler,
    extract::Query,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    cucc::{headers::ImgHeader, sql::get_conn},
    db::data as Data,
};

#[debug_handler]
pub async fn image_get(cucc: Query<ImgHeader>) -> Result<impl IntoResponse, StatusCode> {
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
        let mut content = Vec::new();
        kep_file.read_to_end(&mut content);
        Ok((content))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
