use axum::{debug_handler, extract::Request, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/get", get(items_get))
}

#[debug_handler]
pub async fn items_get(mut request: Request) -> String {
    String::from("Kuki")
}
