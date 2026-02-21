use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    database: Arc<RwLock<Vec<Task>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        database: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .with_state(state); // no extra Arc needed

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Server running at http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello from the Rust Backend!"
}
