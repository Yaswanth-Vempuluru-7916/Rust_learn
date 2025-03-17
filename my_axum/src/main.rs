use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!! </strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("Listening on {addr}");

    let listener =tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,routes_hello).await.unwrap();
}
