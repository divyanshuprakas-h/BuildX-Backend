mod app;
mod models;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = app::create_app();
    let addr = SocketAddr::from(([127,0,0,1], 8000));

    println!("BuildX backend running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind Server Address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
