use axum::{routing::get, Router};

mod base;

pub fn routes() -> Router {
    Router::new().route("/", get(base::user_home))
}
