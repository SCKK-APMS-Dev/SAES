use axum::{middleware, routing::get, Router};

use crate::utils::middle::admin_auth;

mod base;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::admin_home))
        .route("/stat", get(base::admin_stat))
        .layer(middleware::from_fn(admin_auth))
}
