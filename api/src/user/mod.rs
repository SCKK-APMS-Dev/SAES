use axum::Router;

mod base;

pub fn routes() -> Router {
    Router::new().route("/", base::user_home())
}
