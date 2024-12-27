use serde::Deserialize;
use socketioxide::extract::SocketRef;
use stores::get_stores;
use tracing::{info, warn};

use crate::{
    auth::get_discord_envs,
    logging::db_log,
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

pub async fn on_connect(socket: SocketRef, data: InitialData) {
    info!(
        "Socket {:?} connected: {:?} {:?}",
        socket.id,
        socket.ns(),
        data,
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
                    socket.id, tag.name, tag.id,
                );
                db_log(tag.name.clone(), None, None, "LOGIN", None).await;
                let mama = get_stores();
                if tag.admin {
                    socket.join("mv").expect("MV Szobacsatlakozás sikertelen")
                }
                // io.to("socketppl")
                //   .emit("socketppl-update", io.sockets().unwrap().len())
                //   .expect("SocketPPL - Update on connect kiküldése sikertelen");
                socket.join("ucp").expect("UCP Szobacsatlakozás sikertelen");
                socket.emit("maintenance", mama.maintenance).unwrap();
                socket.emit("announcement", mama.announcement).unwrap();
                socket.emit("doneload", "").unwrap();
                //socket.on(
                //    "JoinEvent",
                //   move |s: SocketRef, Data(data): Data<EventData>| {
                //       if data.event_name == "socketppl" {
                //           s.join(data.event_name).unwrap();
                //           s.emit("socketppl-update", iod).expect("Fasz van");
                //      }
                //  },
                // )//;
                // socket.on(
                //     "LeaveEvent",
                //     move |s: SocketRef, Data(data): Data<EventData>| {
                //        if data.event_name == "socketppl" {
                //            s.leave(data.event_name).unwrap();
                //       }
                //    },
                // );
                socket.on_disconnect(move |s: SocketRef| {
                    info!("Socket {} disconnected {} / {}", s.id, tag.name, tag.id);
                    //io.to("socketppl")
                    //   .emit("socketppl-update", iod - 1)
                    //   .expect("SocketPPL - Update on disconnect kiküldése sikertelen");
                });
            } else {
                warn!("Socket {} nincs joga", socket.id);
                return socket.disconnect().unwrap();
            }
        } else {
            warn!("Socket {} érvénytelen lekérés", socket.id);
            return socket.disconnect().unwrap();
        }
    } else {
        warn!("Socket {} érvénytelen lekérés", socket.id);
        return socket.disconnect().unwrap();
    }
}
