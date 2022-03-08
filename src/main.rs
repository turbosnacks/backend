use axum::{routing::get, Router, Server};
use std::net::SocketAddrV4;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let address: SocketAddrV4 = "0.0.0.0:3000".parse().unwrap();
    Server::bind(&address.into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
