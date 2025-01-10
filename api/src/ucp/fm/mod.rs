use axum::{middleware, routing::get, Router};

use crate::utils::middle::fm_auth;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "cs" }))
        .layer(middleware::from_fn(fm_auth))
}
