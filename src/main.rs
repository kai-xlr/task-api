use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
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

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        database: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task))
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

/// GET /tasks
async fn get_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let db = state.database.read().await;
    Json(db.clone())
}

/// GET /tasks/:id
async fn get_task(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Task>, (StatusCode, &'static str)> {
    let db = state.database.read().await;
    if let Some(task) = db.iter().find(|t| t.id == id).cloned() {
        Ok(Json(task))
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found"))
    }
}

/// POST /tasks
async fn create_task(State(state): State<AppState>, Json(payload): Json<CreateTask>) -> Json<Task> {
    let mut db = state.database.write().await;

    let new_task = Task {
        id: db.len() as i32 + 1, // simple ID generation
        title: payload.title,
        completed: false,
    };

    db.push(new_task.clone());

    Json(new_task)
}
