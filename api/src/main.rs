use axum::{response::Redirect, routing::get, Router};
use dotenvy::dotenv;
use image::{image_get, leintes_image_get};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

mod auth;
mod db;
mod image;
mod list;
mod shorts;
mod user;
mod utils;

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
        .route("/list", get(list::list_get))
        .route("/limg", get(leintes_image_get))
        .route("/shorts", get(shorts::get_shorts))
        .nest("/user", user::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
