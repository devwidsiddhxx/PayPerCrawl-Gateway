use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "200 OK\n\nWelcome to Demo Blog\n" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Demo website listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
