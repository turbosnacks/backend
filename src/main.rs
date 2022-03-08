use axum::{routing::get, Router, Server};
use std::net::{Ipv4Addr, SocketAddr};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    Server::bind(&address.into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
