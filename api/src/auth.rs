use std::env;

use axum::extract::Query;
use axum::{debug_handler, response::Redirect};
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};
use url_builder::URLBuilder;

use serde::Deserialize;

pub struct DiscordStuff {
    pub api_endpoint: String,
    pub discord_base: String,
    pub discord_id: String,
    pub discord_secret: String,
    pub redirect_url: String,
    pub domain: String,
    pub fdomain: String,
    pub secret_key: String,
}

pub fn get_discord_envs() -> DiscordStuff {
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
    DiscordStuff {
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

pub fn get_auth_url() -> String {
    let mut ub = URLBuilder::new();
    let ds = get_discord_envs();
    ub.set_protocol("https")
        .set_host(&ds.discord_base.as_str())
        .add_param("response_type", "code")
        .add_param("client_id", &ds.discord_id)
        .add_param("scope", "identify")
        .add_param("redirect_uri", &ds.redirect_url);
    ub.build()
}

#[derive(Deserialize)]
pub struct Code {
    code: String,
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
    let client = reqwest::Client::new();
    let ds = get_discord_envs();
    let data = [
        ("grant_type", "authorization_code"),
        ("code", &query.code),
        ("redirect_uri", &ds.redirect_url),
    ];
    let url = format!("{}/oauth2/token", ds.api_endpoint);
    let response = client
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
    cookies.add(
        Cookie::build(("auth_token", object.access_token))
            .max_age(Duration::seconds(object.expires_in))
            .domain(ds.domain.clone())
            .path("/")
            .build(),
    );

    let red = Redirect::to(&ds.fdomain);
    red
}
