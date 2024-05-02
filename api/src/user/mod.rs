use axum::{middleware, routing::get, Router};

use crate::cucc::middle::basic_auth;

mod base;

pub fn routes() -> Router {
    Router::new().route(
        "/",
        get(base::user_home).layer(middleware::from_fn(basic_auth)),
    )
}
