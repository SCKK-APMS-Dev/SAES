use axum::{middleware, routing::get, Router};

use crate::utils::middle::fm_auth;

mod base;
mod logs;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::fm_home))
        .nest("/logs", logs::get_routes())
        .layer(middleware::from_fn(fm_auth))
}
