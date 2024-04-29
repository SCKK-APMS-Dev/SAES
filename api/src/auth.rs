use axum::extract::Query;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use serde::Deserialize;
use std::env;

pub fn get_url() -> String {
    let id = env::var("DISCORD_ID").expect("Discord ID lekérése sikertelen");
    let secret = env::var("DISCORD_SECRET").expect("Discord Secret lekérése sikertelen");
    let cb = env::var("REDIRECT_URL").expect("Redirect URL lekérése sikertelen");
    let client = BasicClient::new(
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
    .set_redirect_uri(RedirectUrl::new(cb).expect("Redirect Url lekérése sikertelen"));

    // Generate a PKCE challenge.
    let pkce = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let auth_url = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce.0)
        .url();
    auth_url.0.to_string()
}

struct Code {
    code: String,
}

pub async fn callback(code: Query<Code>) -> String {
    code.0.code
}
