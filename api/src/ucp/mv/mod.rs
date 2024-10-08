use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::utils::middle::admin_auth;

mod base;
mod home;
mod items;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::admin_home))
        .route("/home", get(home::admin_home_stat))
        .route("/stat", get(base::admin_stat))
        .route("/get", get(items::admin_items_get))
        .route("/post", post(items::admin_items_post))
        .layer(middleware::from_fn(admin_auth))
}
