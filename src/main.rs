use std::net::SocketAddr;

use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/greet/{username}", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(format!("{}", addr))
        .await
        .unwrap();

    println!("[{}] Listening ...", addr);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn health() -> &'static str {
    "Healthy!"
}

async fn greet(Path(username): Path<String>) -> String {
    format!("Hello, {username}!")
}
