use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::utils::middle::mv_auth;

mod base;
mod home;
mod items;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::mv_home))
        .route("/home", get(home::mv_home_stat))
        .route("/stat", get(base::mv_stat))
        .route("/get", get(items::mv_items_get))
        .route("/post", post(items::mv_items_post))
        .layer(middleware::from_fn(mv_auth))
}
