use std::env;

use axum::debug_handler;
use axum::extract::Query;
use url_builder::URLBuilder;

use serde::Deserialize;

struct DiscordStuff {
    discord_base: String,
    discord_token_url: String,
    discord_id: String,
    discord_secret: String,
    redirect_url: String,
    api_endpoint: String,
}

fn get_discord_envs() -> DiscordStuff {
    let id = env::var("DISCORD_ID")
        .expect("DISCORD_ID .env fájlból betöltése sikertelen. Létre van hozva?");
    let secret = env::var("DISCORD_SECRET")
        .expect("DISCORD_SECRET .env fájlból betöltése sikertelen. Létre van hozva?");
    let cb = env::var("REDIRECT_URL")
        .expect("REDIRECT_URL .env fájlból betöltése sikertelen. Létre van hozva?");
    DiscordStuff {
        discord_id: id,
        discord_secret: secret,
        redirect_url: cb,
        discord_base: String::from("discord.com/oauth2/authorize"),
        discord_token_url: String::from("discord.com/api/oauth2/token"),
        api_endpoint: String::from("https://discord.com/api/v10"),
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
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
    access_token: String,
    // Add other relevant fields from the response here (e.g., token_type, expires_in)
}

#[debug_handler]
pub async fn callback(Query(query): Query<Code>) -> String {
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
        .expect("Átalakítás sikerleten");
    let object: TokenResponse =
        serde_json::from_str(&token_response).expect("Átalakítás sikertelen");
    println!("{},{}", object.access_token, object.expires_in);
    token_response
}
