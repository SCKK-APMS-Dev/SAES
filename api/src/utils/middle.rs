use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::auth::get_discord_envs;

use super::{
    api::get_api_envs,
    permissions::{get_perm, Factions, Permissions},
};

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FactionRecord {
    pub factionid: i8,
    pub factionname: String,
    pub positionid: i8,
    pub positionname: String,
    pub shiftid: i8,
    pub shiftname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetUserRes {
    pub factionrecords: Vec<FactionRecord>,
    pub issysadmin: bool,
    pub permissions: Vec<String>,
    pub userid: i8,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct SAMTAuth {
    pub userdiscordid: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Driver {
    pub discordid: String,
    pub driverid: i8,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
    pub taxi: Option<FactionRecord>,
    pub tow: Option<FactionRecord>,
}

pub async fn ucp_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = headers.get("cookie");
    let ds = get_discord_envs();
    let envs = get_api_envs();
    if auth.is_some() {
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
                    .post(format!("{}/saes/authenticate", envs.samt))
                    .json(&SAMTAuth {
                        userdiscordid: real_user.id.clone(),
                    })
                    .basic_auth("dev", envs.testpass)
                    .send()
                    .await
                    .expect("Lekérés sikertelen")
                    .text()
                    .await
                    .expect("Átalakítás sikertelen");
                let parsed_tag = serde_json::from_str(&getuser);
                if parsed_tag.is_ok() {
                    let real_tag: GetUserRes = parsed_tag.unwrap();
                    if real_tag
                        .permissions
                        .contains(&get_perm(Permissions::SaesLogin))
                        || real_tag.issysadmin
                    {
                        let tag = Driver {
                            discordid: real_user.id,
                            name: real_tag.username,
                            driverid: real_tag.userid,
                            admin: real_tag.issysadmin,
                            perms: real_tag.permissions,
                            taxi: real_tag
                                .factionrecords
                                .iter()
                                .find(|fact| fact.factionid == 1)
                                .cloned(),
                            tow: real_tag
                                .factionrecords
                                .iter()
                                .find(|fact| fact.factionid == 3)
                                .cloned(),
                        };
                        request.extensions_mut().insert(tag);
                        return Ok(next.run(request).await);
                    } else {
                        return Err((
                            StatusCode::FORBIDDEN,
                            "Nincs jogod a belépéshez! (samt.login)".to_string(),
                        ));
                    }
                } else {
                    println!("{:?}", parsed_tag);
                    return Err((StatusCode::FORBIDDEN, "Nincs jogod ehhez!".to_string()));
                }
            } else {
                return Err((StatusCode::BAD_REQUEST, "Érvénytelen lekérés!".to_string()));
            }
        } else {
            return Ok(Redirect::to("/auth").into_response());
        }
    } else {
        return Err((StatusCode::NOT_FOUND, "Nincs kuki".to_string()));
    };
}

pub async fn sm_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    let fact = if uwrp.taxi.is_some() {
        uwrp.perms
            .contains(&get_perm(Permissions::SaesSm(Factions::SCKK)))
    } else if uwrp.tow.is_some() {
        uwrp.perms
            .contains(&get_perm(Permissions::SaesSm(Factions::TOW)))
    } else {
        false
    };
    if uwrp.admin == true || fact {
        return Ok(next.run(req).await);
    } else {
        return Err((StatusCode::FORBIDDEN, "Nem vagy műszakvezető".to_string()));
    }
}
