use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef};
use stores::get_stores;
use tracing::info;

use crate::{
    auth::get_discord_envs,
    utils::{
        api::get_api_envs,
        middle::{DiscordUser, GetUserRes, Tag},
    },
};

mod stores;

#[derive(Debug, Deserialize)]
pub struct InitialData {
    auth_token: String,
}

pub async fn on_connect(socket: SocketRef, Data(data): Data<InitialData>) {
    info!(
        "Socket.IO connected: {:?} {:?} {:?}",
        socket.ns(),
        socket.id,
        data
    );
    let client = reqwest::Client::new();
    let ds = get_discord_envs();
    let envs = get_api_envs();
    let dcuserget = client
        .get(format!("{}/users/@me", ds.api_endpoint))
        .header("Authorization", format!("Bearer {}", data.auth_token))
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
                info!(
                    "Socket {} authenticated: {} / {}",
                    socket.id, tag.id, tag.name
                );
                let mama = get_stores();
                socket.emit("maintenance", mama.maintenance).unwrap();
                socket.emit("announcement", mama.announcement).unwrap();
            } else {
                info!("Socket {} nincs joga", socket.id);
                return socket.disconnect().unwrap();
            }
        } else {
            info!("Socket {} érvénytelen lekérés", socket.id);
            return socket.disconnect().unwrap();
        }
    } else {
        info!("Socket {} érvénytelen lekérés", socket.id);
        return socket.disconnect().unwrap();
    }
}
