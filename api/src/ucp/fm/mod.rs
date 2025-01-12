use axum::{middleware, routing::get, Router};

use crate::utils::middle::fm_auth;

mod base;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::fm_home))
        .layer(middleware::from_fn(fm_auth))
}
