use std::path::PathBuf;

use gcp_auth::{CustomServiceAccount, TokenProvider};

pub async fn get_google_auth() -> String {
    let credentials_path = PathBuf::from("google-service-account.json");
    let service_account = CustomServiceAccount::from_file(credentials_path)
        .expect("Google Service Account létrehozása sikertelen");
    let scopes = &[
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/spreadsheets",
    ];
    let token = service_account
        .token(scopes)
        .await
        .expect("Token lekérése sikertelen");
    token.as_str().to_owned()
}
