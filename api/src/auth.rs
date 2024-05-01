use axum::extract::Query;
use axum::{debug_handler, Json};
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, RedirectUrl, RefreshToken, RevocationErrorResponseType, Scope,
    StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::env;
use std::time::Duration;

fn get_client() -> oauth2::Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
> {
    let id = env::var("DISCORD_ID")
        .expect("DISCORD_ID .env fájlból betöltése sikertelen. Létre van hozva?");
    let secret = env::var("DISCORD_SECRET")
        .expect("DISCORD_SECRET .env fájlból betöltése sikertelen. Létre van hozva?");
    let cb = env::var("REDIRECT_URL")
        .expect("REDIRECT_URL .env fájlból betöltése sikertelen. Létre van hozva?");
    return BasicClient::new(
        ClientId::new(id),
        Some(ClientSecret::new(secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
            .expect("Auth Url lekérése sikertelen"),
        Some(
            TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
                .expect("Token Url lekérése sikertelen"),
        ),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(cb).expect("Redirect Url megadása sikertelen"));
}

pub fn get_url() -> String {
    // Generate a PKCE challenge.
    let client = get_client();
    // Generate the full authorization URL.
    let auth_url = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        // Set the PKCE code challenge.
        .url();
    auth_url.0.to_string()
}

#[derive(Deserialize)]
pub struct Code {
    code: String,
}

#[debug_handler]
pub async fn callback(Query(query): Query<Code>) -> String {
    let client = get_client();
    let result = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request(http_client)
        .expect("Token lekérése sikertelen");
    String::from("kuki")
}
