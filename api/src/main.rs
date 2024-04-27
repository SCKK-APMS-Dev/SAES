use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;

mod user;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "SCKK Web API V2 Axum & SQLx használatával" }),
        )
        .nest("/user", user::routes())
        .layer(CookieManagerLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
