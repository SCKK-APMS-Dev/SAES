use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use sea_orm::EntityTrait;

use crate::{
    auth::get_discord_envs,
    db::shorts,
    utils::{queries::ShortsQuery, sql::get_conn},
};

#[debug_handler]
pub async fn get_shorts(
    quer: Query<ShortsQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let key = get_discord_envs();
    if quer.key == key.secret_key {
        let db = get_conn().await;
        let shorts = shorts::Entity::find()
            .all(&db)
            .await
            .expect("[SHORTS:ERROR] Lekérés sikertelen");
        Ok(Json(shorts))
    } else {
        Err((StatusCode::FORBIDDEN, "Ehhez nincs jogod!".to_string()))
    }
}
