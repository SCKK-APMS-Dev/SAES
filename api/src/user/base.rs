use tower_cookies::Cookies;

pub async fn user_home(cookies: Cookies) -> String {
    if let Some(token) = cookies.get("auth_token") {
        format!("Van token: {}", token.value())
    } else {
        String::from("Nincs token")
    }
}
