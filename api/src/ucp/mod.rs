use axum::{middleware, routing::get, Router};

use crate::utils::middle::ucp_auth;

mod base;
mod calls;
mod items;
mod mv;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::ucp_home))
        .route("/types", get(base::ucp_get_types))
        .route("/statuses", get(base::ucp_get_statuses))
        .route("/calls", get(calls::ucp_calls))
        .nest("/items", items::routes())
        .nest("/mv", mv::routes())
        .layer(middleware::from_fn(ucp_auth))
}
