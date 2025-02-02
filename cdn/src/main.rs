use axum::{routing::get, Router};
use dotenvy::dotenv;
use image::image_get;
use saes_shared::sql::get_db_conn;
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod envs;
mod image;

pub static DB_CLIENT: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    envs::load_envs().await;
    DB_CLIENT.set(get_db_conn().await).unwrap();
    let app = Router::new()
        .route("/", get(|| async { "SAES CDN server" }))
        .route("/get", get(image_get))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(TraceLayer::new_for_http());
    info!("Server runs on :3100");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
