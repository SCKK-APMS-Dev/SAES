use axum::routing::{get, MethodRouter};

pub fn user_home() -> MethodRouter {
    get(|| async { "kuki" })
}
