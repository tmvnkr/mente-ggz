use axum::{
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, IntoMakeService},
    Json, Router, Server,
};
use hyper::{header, server::conn::AddrIncoming, Result};
use std::net::{SocketAddr, TcpListener};
use tower_http::cors::CorsLayer;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener, client_addr: SocketAddr) -> Result<App> {
    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .route("/test", get(json))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
                .allow_headers(vec![header::CONTENT_TYPE]),
        );

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}

async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}
