use axum::{routing::get, Router};
use dotenvy::dotenv;
use image::image_get;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod envs;
mod image;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    envs::load_envs().await;
    let app = Router::new()
        .route("/", get(|| async { "SAES Static server" }))
        .route("/get", get(image_get))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(TraceLayer::new_for_http());
    info!("Server runs on :3100");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
