use axum::{debug_handler, response::IntoResponse, Json};
use http::{HeaderMap, StatusCode};
use saes_shared::{db::shorts, sql::get_db_conn};

use sea_orm::EntityTrait;

use crate::auth::get_discord_envs;

#[debug_handler]
pub async fn base_get_shorts(
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let key = get_discord_envs();
    let headerkey = headers.get("secret-key");
    if headerkey.is_some() {
        if headerkey.unwrap().to_str().unwrap() == key.secret_key {
            let db = get_db_conn().await;
            let shorts = shorts::Entity::find()
                .all(&db)
                .await
                .expect("[SHORTS:ERROR] Lekérés sikertelen");
            Ok(Json(shorts))
        } else {
            Err((StatusCode::FORBIDDEN, "Ehhez nincs jogod!".to_string()))
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, "Nincs secret-key!".to_string()))
    }
}
