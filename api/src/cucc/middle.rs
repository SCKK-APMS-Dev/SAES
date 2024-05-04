use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse, Extension};
use reqwest::StatusCode;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::auth::get_discord_envs;

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
    pub id: String,
}

pub async fn basic_auth(
    Extension(cookies): Extension<Cookies>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // do something with `request`...
    let auth = cookies.get("auth_token");
    let ds = get_discord_envs();
    if auth.is_some() {
        let client = reqwest::Client::new();
        let dcuserget: String = client
            .get(format!("{}/users/@me", ds.api_endpoint))
            .header("Authorization", format!("Bearer {}", auth.unwrap().value()))
            .send()
            .await
            .expect("Lekérés sikertelen")
            .text()
            .await
            .expect("Átalakítás sikertelen");
        let parsed_user: DiscordUser =
            serde_json::from_str(&dcuserget).expect("User object létrehozása sikertelen");
        request.extensions_mut().insert(parsed_user.id);
        return Ok(next.run(request).await);
    } else {
        return Err((StatusCode::NOT_FOUND, "Nincs kuki".to_string()));
    };
}
