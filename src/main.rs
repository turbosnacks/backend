use axum::{handler::Handler, routing::get, Router, Server};
use std::net::{Ipv4Addr, SocketAddr};

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(routes::healthcheck))
        .fallback(routes::fallback.into_service());
    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    Server::bind(&address.into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
