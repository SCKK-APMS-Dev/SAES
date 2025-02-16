use axum::{debug_handler, response::IntoResponse, Json};
use http::StatusCode;

use crate::{
    config::{loader::write_config, structs::GlobalConfig},
    MAIN_CONFIG,
};

#[debug_handler]
pub async fn sys_get_configs() -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = MAIN_CONFIG.get().unwrap();
    return Ok(Json(config));
}

#[debug_handler]
pub async fn sys_post_global_config(
    Json(c): Json<GlobalConfig>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = MAIN_CONFIG.get().unwrap();
    let mut new_config = conf.clone();
    if c.announcement.is_some() {
        if c.announcement != conf.global.announcement {
            new_config.global.announcement = c.announcement;
        }
    }
    if c.maintenance.is_some() {
        if c.maintenance != conf.global.maintenance {
            new_config.global.maintenance = c.maintenance;
        }
    }
    write_config(&new_config);
    Ok(Json(new_config.global))
}
