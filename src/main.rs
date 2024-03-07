use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Json, Router,
};

pub use self::error::{Error, Result};

use crate::model::{ModelController, Todo, TodoBody};

mod error;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let app = Router::new().nest("/", routes(mc.clone()));

    // let app = Router::new().route("/", get(|| async { "Hello, Rust for hompage hshshhs" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("{:?}", listener.local_addr());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn create_todo(
    mc: State<ModelController>,
    Json(todo_body): Json<TodoBody>,
) -> Result<Json<Todo>> {
    let todo = mc.create_todo(todo_body).await?;

    Ok(Json(todo))
}

async fn list_todos(mc: State<ModelController>) -> Result<Json<Vec<Todo>>> {
    let todos = mc.list_todos().await?;

    Ok(Json(todos))
}

async fn delete_todo(mc: State<ModelController>, Path(id): Path<u64>) -> Result<Json<Todo>> {
    let todo = mc.delete_todo(id).await?;

    Ok(Json(todo))
}

fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(mc)
}

// fn main() {
//     println!("Hello, world!");
// }
