use axum::{response::Redirect, routing::get, Router};
use dotenvy::dotenv;
use image::image_get;
use tower_cookies::CookieManagerLayer;

mod auth;
mod cucc;
mod db;
mod image;
mod user;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    let app = Router::new()
        .route(
            "/",
            get(|| async { "SCKK Web API V2 Axum & SQLx használatával" }),
        )
        .route(
            "/auth",
            get(|| async { Redirect::to(&auth::get_auth_url()) }),
        )
        .route("/cb", get(auth::callback))
        .route("/img", get(image_get))
        .nest("/user", user::routes())
        .layer(CookieManagerLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
