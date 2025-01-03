use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::utils::middle::sm_auth;

mod base;
mod home;
mod items;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::sm_home))
        .route("/home", get(home::sm_home_stat))
        .route("/stat", get(base::sm_stat))
        .route("/get", get(items::sm_items_get))
        .route("/post", post(items::sm_items_post))
        .layer(middleware::from_fn(sm_auth))
}
