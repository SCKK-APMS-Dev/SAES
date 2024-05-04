use axum::{debug_handler, extract::Request};
use tower_cookies::Cookies;

use crate::cucc::middle::DiscordUser;

#[debug_handler]
pub async fn user_home(cookies: Cookies, mut request: Request) -> String {
    let exts: Option<&String> = request.extensions_mut().get();
    String::from(exts.unwrap())
}
