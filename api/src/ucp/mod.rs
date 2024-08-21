use axum::{middleware, routing::get, Router};

use crate::utils::middle::basic_auth;

mod base;
mod calls;
mod items;
mod mv;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::user_home))
        .route("/calls", get(calls::calls))
        .nest("/items", items::routes())
        .nest("/mv", mv::routes())
        .layer(middleware::from_fn(basic_auth))
}
