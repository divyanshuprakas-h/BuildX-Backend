mod app;
mod config;
mod docs;
mod models;
mod routes;
mod services;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = app::create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("BuildX backend running on http://{}", addr);
    println!("Swagger Docs running on http://127.0.0.1:8000/docs");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind Server Address");

    axum::serve(listener, app).await.expect("Server failed");
}
