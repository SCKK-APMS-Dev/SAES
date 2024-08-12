#![allow(non_snake_case)]

use axum::{extract::Request, http::HeaderMap, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::auth::get_discord_envs;

use super::api::get_api_envs;

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetUserRes {
    pub PermissionGroup: Option<u32>,
    pub PlayerName: String,
    pub PositionId: i8,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub admin: bool,
    pub am: bool,
}

pub async fn basic_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // do something with `request`...
    let special_key = headers.get("secret-key");
    let special_id = headers.get("secret-id");
    let auth = headers.get("cookie");
    let ds = get_discord_envs();
    let envs = get_api_envs();
    if special_key.is_some()
        && special_id.is_some()
        && special_key.unwrap().to_str().unwrap() == ds.secret_key
    {
        let client = reqwest::Client::new();
        let getuser: String = client
            .get(format!(
                "{}/appauth/login/{}",
                envs.patrik,
                special_id.unwrap().to_str().unwrap()
            ))
            .send()
            .await
            .expect("Lekérés sikertelen")
            .text()
            .await
            .expect("Átalakítás sikertelen");
        let parsed_tag = serde_json::from_str(&getuser);
        if parsed_tag.is_ok() {
            let real_tag: GetUserRes = parsed_tag.unwrap();
            let am_admins: [i8; 10] = [35, 36, 37, 43, 44, 45, 46, 47, 48, 49];
            let tag = Tag {
                id: special_id.unwrap().to_str().unwrap().to_string(),
                name: real_tag.PlayerName,
                admin: if real_tag.PermissionGroup.is_some_and(|x| x == 1)
                    || am_admins.contains(&real_tag.PositionId)
                {
                    true
                } else {
                    false
                },
                am: if real_tag.PositionId.gt(&34) && real_tag.PositionId.lt(&50) {
                    true
                } else {
                    false
                },
            };
            request.extensions_mut().insert(tag);
            return Ok(next.run(request).await);
        } else {
            return Err((StatusCode::FORBIDDEN, "Nincs jogod!".to_string()));
        }
    } else if auth.is_some() {
        let client = reqwest::Client::new();
        let dcuserget = client
            .get(format!("{}/users/@me", ds.api_endpoint))
            .header(
                "Authorization",
                format!("Bearer {}", auth.unwrap().to_str().unwrap()),
            )
            .send()
            .await
            .expect("Lekérés sikertelen");
        if dcuserget.status().as_u16() == 200 {
            let handled_user = dcuserget.text().await.expect("Átalakítás sikertelen");
            let parsed_user = serde_json::from_str(&handled_user);
            if parsed_user.is_ok() {
                let real_user: DiscordUser = parsed_user.unwrap();
                let getuser: String = client
                    .get(format!("{}/appauth/login/{}", envs.patrik, real_user.id))
                    .send()
                    .await
                    .expect("Lekérés sikertelen")
                    .text()
                    .await
                    .expect("Átalakítás sikertelen");
                let parsed_tag = serde_json::from_str(&getuser);
                if parsed_tag.is_ok() {
                    let real_tag: GetUserRes = parsed_tag.unwrap();
                    let am_admins: [i8; 10] = [35, 36, 37, 43, 44, 45, 46, 47, 48, 49];
                    let tag = Tag {
                        id: real_user.id,
                        name: real_tag.PlayerName,
                        admin: if real_tag.PermissionGroup.is_some_and(|x| x == 1)
                            || am_admins.contains(&real_tag.PositionId)
                        {
                            true
                        } else {
                            false
                        },
                        am: if real_tag.PositionId.gt(&34) && real_tag.PositionId.lt(&50) {
                            true
                        } else {
                            false
                        },
                    };
                    request.extensions_mut().insert(tag);
                    return Ok(next.run(request).await);
                } else {
                    return Err((StatusCode::FORBIDDEN, "Nincs jogod!".to_string()));
                }
            } else {
                return Err((StatusCode::BAD_REQUEST, "Érvénytelen lekérés!".to_string()));
            }
        } else {
            return Err((StatusCode::NOT_ACCEPTABLE, "Sikertelen lekérés".to_string()));
        }
    } else {
        return Err((StatusCode::NOT_FOUND, "Nincs kuki".to_string()));
    };
}

pub async fn admin_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Tag> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, basic_auth megtörtént?");
    if uwrp.admin == true {
        return Ok(next.run(req).await);
    } else {
        return Err((StatusCode::FORBIDDEN, "Nem vagy admin".to_string()));
    }
}
