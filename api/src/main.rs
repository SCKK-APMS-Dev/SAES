use std::env;

use axum::{routing::get, Router};
use dotenvy::dotenv;
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
    let env_mode = env::var("ENV_MODE");
    if env_mode.is_err() {
        panic!("ENV_MODE nincs setelve! production / testing / devel")
    }
    if ![
        "production".to_string(),
        "testing".to_string(),
        "devel".to_string(),
    ]
    .contains(&env_mode.clone().unwrap())
    {
        panic!("ENV_MODE rosszul setelve! production / testing / devel")
    } else {
        info!("Running in {} mode", env_mode.unwrap().to_uppercase());
    }
    init::main();
    io.ns(
        "/",
        move |socket: SocketRef, Data(data): Data<InitialData>| socket::on_connect(socket, data),
    );
    let app = Router::new()
        .route(
            "/",
            get(|| async { "SAES API V2 Axum & Sea-ORM használatával" }),
        )
        .route("/auth", get(auth::auth_home))
        .route("/auth/cb", get(auth::base_callback))
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
    info!("Server runs on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
