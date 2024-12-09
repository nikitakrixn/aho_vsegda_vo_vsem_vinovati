mod models;

use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {

    let addr = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::serve(addr, app)
        .await
        .unwrap();
}