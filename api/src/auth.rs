use std::env;
use std::sync::RwLock;

use axum::extract::Query;
use axum::{debug_handler, response::Redirect};
use base64::engine::general_purpose;
use base64::Engine;
use lazy_static::lazy_static;
use random_string::{charsets, generate};
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};
use url_builder::URLBuilder;

use serde::{Deserialize, Serialize};

use crate::WEB_CLIENT;

lazy_static! {
    static ref GLOBAL_ARRAY: RwLock<Vec<String>> = RwLock::new(Vec::new());
}
pub struct DiscordAuth {
    pub api_endpoint: String,
    pub discord_base: String,
    pub discord_id: String,
    pub discord_secret: String,
    pub redirect_url: String,
    pub domain: String,
    pub fdomain: String,
    pub secret_key: String,
}

pub fn get_discord_envs() -> DiscordAuth {
    let id = env::var("DISCORD_ID")
        .expect("DISCORD_ID .env fájlból betöltése sikertelen. Létre van hozva?");
    let secret = env::var("DISCORD_SECRET")
        .expect("DISCORD_SECRET .env fájlból betöltése sikertelen. Létre van hozva?");
    let cb = env::var("REDIRECT_URL")
        .expect("REDIRECT_URL .env fájlból betöltése sikertelen. Létre van hozva?");
    let domain =
        env::var("DOMAIN").expect("DOMAIN .env fájlból betöltése sikertelen. Létre van hozva?");
    let fdomain = env::var("FULL_DOMAIN")
        .expect("FULL_DOMAIN .env fájlból betöltése sikertelen. Létre van hozva?");
    let secret_key = env::var("SECRET_KEY")
        .expect("SECRET_KEY .env fájlból betöltése sikertelen. Létre van hozva?");
    DiscordAuth {
        api_endpoint: String::from("https://discord.com/api/v10"),
        discord_id: id,
        discord_secret: secret,
        domain,
        fdomain,
        redirect_url: cb,
        discord_base: String::from("discord.com/oauth2/authorize"),
        secret_key,
    }
}

#[derive(Deserialize)]
pub struct Code {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    expires_in: i64,
    // refresh_token: String,
    access_token: String,
    // Add other relevant fields from the response here (e.g., token_type, expires_in)
}

#[debug_handler]
pub async fn base_callback(Query(query): Query<Code>, cookies: Cookies) -> Redirect {
    let ds = get_discord_envs();
    let data = [
        ("grant_type", "authorization_code"),
        ("code", &query.code),
        ("redirect_uri", &ds.redirect_url),
    ];
    let url = format!("{}/oauth2/token", ds.api_endpoint);
    let response = WEB_CLIENT
        .post(&url)
        .basic_auth(ds.discord_id, Some(ds.discord_secret.to_string()))
        .form(&data)
        .send()
        .await;
    let token_response: String = response
        .expect("Lekérés sikertelen")
        .text()
        .await
        .expect("Átalakítás sikertelen");
    let object: TokenResponse =
        serde_json::from_str(&token_response).expect("Átalakítás sikertelen");
    let path = String::from_utf8(general_purpose::STANDARD.decode(query.state).unwrap()).unwrap();
    let path_full: AuthState = serde_json::from_str(&path).expect("Nem megy");
    if GLOBAL_ARRAY.read().unwrap().contains(&path_full.truestate) {
        cookies.add(
            Cookie::build(("auth_token", object.access_token))
                .max_age(Duration::seconds(object.expires_in))
                .http_only(true)
                .secure(true)
                .domain(ds.domain.clone())
                .path("/")
                .build(),
        );
        let id = GLOBAL_ARRAY
            .read()
            .unwrap()
            .iter()
            .position(|x| x == &path_full.truestate)
            .unwrap();
        GLOBAL_ARRAY.write().unwrap().remove(id);
        return Redirect::to(&format!("{}{}", &ds.fdomain, path_full.path));
    }
    Redirect::to("google.com")
}

fn base_path() -> String {
    "/ucp".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthHomeCode {
    #[serde(default = "base_path")]
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthState {
    path: String,
    truestate: String,
}

#[debug_handler]
pub async fn auth_home(Query(q): Query<AuthHomeCode>) -> Redirect {
    let mut ub = URLBuilder::new();
    let rstate = generate(256, charsets::ALPHANUMERIC);
    let state = AuthState {
        path: q.path,
        truestate: rstate.clone(),
    };
    GLOBAL_ARRAY.write().unwrap().push(rstate);
    let state_str = serde_json::to_string(&state).expect("Sikertelen átalakítás");
    let ds = get_discord_envs();
    ub.set_protocol("https")
        .set_host(&ds.discord_base.as_str())
        .add_param("response_type", "code")
        .add_param("state", &general_purpose::STANDARD.encode(state_str))
        .add_param("client_id", &ds.discord_id)
        .add_param("scope", "identify")
        .add_param("prompt", "none")
        .add_param("redirect_uri", &ds.redirect_url);
    let built_url = ub.build();
    Redirect::to(&built_url)
}
