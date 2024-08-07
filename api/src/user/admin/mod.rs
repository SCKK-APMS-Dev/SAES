use axum::{middleware, routing::get, Router};

use crate::utils::middle::admin_auth;

mod base;
mod items;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::admin_home))
        .route("/stat", get(base::admin_stat))
        .route("/items/get", get(items::admin_items_get))
        .layer(middleware::from_fn(admin_auth))
}
