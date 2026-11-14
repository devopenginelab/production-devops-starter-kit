use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[tokio::main]
async fn main() {
    // Define the /health endpoint used by load balancers and container orchestrators
    let app = Router::new().route("/health", get(health_check));

    // Bind to 0.0.0.0 (all network interfaces) instead of 127.0.0.1 (localhost only)
    // CRITICAL FOR DOCKER: If you bind to 127.0.0.1, external traffic cannot reach the container!
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Status API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    })
}