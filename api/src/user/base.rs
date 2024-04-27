use tower_cookies::{Cookie, Cookies};

pub async fn user_home(cookies: Cookies) -> String {
    if let Some(token) = cookies.get("auth_token") {
        format!("Van token: {}", token.value())
    } else {
        cookies.add(Cookie::new("auth_token", "faszos"));
        "Nincs token".to_string()
    }
}
