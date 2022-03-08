use axum::{handler::Handler, routing::get, Router, Server};
use std::net::{Ipv4Addr, SocketAddr};

mod routes;

fn app() -> Router {
    Router::new()
        .route("/health", get(routes::healthcheck))
        .fallback(routes::fallback.into_service())
}

#[tokio::main]
async fn main() {
    let app = app();
    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, HttpBody};
    use axum::http::{Request, Response, StatusCode};
    use std::fmt::Debug;
    use tower::ServiceExt;

    async fn read_body<T>(response: Response<T>) -> String
    where
        T: HttpBody,
        T::Error: Debug,
    {
        let bytes_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let string_body = String::from_utf8(bytes_body.to_vec()).unwrap();
        string_body
    }

    #[tokio::test]
    async fn healthcheck() {
        let app = app();

        let response = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn fallback() {
        let app = app();

        let response = app
            .oneshot(Request::get("/foobar").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = read_body(response).await;
        assert_eq!(&body, "No route found for /foobar");
    }
}
