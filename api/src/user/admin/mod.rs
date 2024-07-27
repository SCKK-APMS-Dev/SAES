use axum::{middleware, routing::get, Router};

use crate::utils::middle::admin_auth;

mod base;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::admin_home))
        .layer(middleware::from_fn(admin_auth))
}
