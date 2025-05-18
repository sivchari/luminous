use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root_handler() -> String {
    "Hello World".to_string()
}
