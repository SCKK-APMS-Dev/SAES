use axum::{routing::get, Router};
use dotenvy::dotenv;
use image::base_image_get;
use socket::InitialData;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod auth;
mod db;
mod image;
mod init;
mod list;
mod logging;
mod shorts;
mod socket;
mod ucp;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let (layer, io) = SocketIo::new_layer();
    init::main();
    io.ns(
        "/",
        move |socket: SocketRef, Data(data): Data<InitialData>| socket::on_connect(socket, data),
    );
    let app = Router::new()
        .route(
            "/",
            get(|| async { "SAES API V2 Axum & SQLx használatával" }),
        )
        .route("/auth", get(auth::auth_home))
        .route("/auth/cb", get(auth::base_callback))
        .route("/img", get(base_image_get))
        .route("/list", get(list::base_list_get))
        .route("/shorts", get(shorts::base_get_shorts))
        .nest("/ucp", ucp::routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());
    // run our app with hyper, listening globally on port 3000
    info!("Szerver indul");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
