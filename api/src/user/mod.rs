use axum::{middleware, routing::get, Router};

use crate::cucc::middle::basic_auth;

mod admin;
mod base;
mod calls;
mod items;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::user_home))
        .route("/calls", get(calls::calls))
        .nest("/items", items::routes())
        .nest("/admin", admin::routes())
        .layer(middleware::from_fn(basic_auth))
}
