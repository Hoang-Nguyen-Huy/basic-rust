use axum:: {
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;
use std::net::SocketAddr;

mod config;

mod models;
use models::{CreateTask, Task};

const SCHEMA: &str = include_str!("../schema.sql");

#[tokio::main]
async fn main () {
    let pool = SqlitePool::connect(config::DATABASE_URL).await.unwrap();

    sqlx::query(SCHEMA).execute(&pool).await.unwrap();

    let app = Router::new()
            .route("/", get(hello))
            .route("/tasks", get(get_tasks))
            .route("/tasks", post(create_task));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}" , addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

}

async fn hello () -> &'static str {
    "Hello this is basic CRUD with Rust"
}

async fn create_task (
    Json(task): Json<CreateTask>,
    State(pool): State<SqlitePool>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let id = Uuid::new_v4();
    let result = sqlx::query!(
        r#"INSERT INTO task(id, title, completed) VALUES(?1, ?2, ?3)"#,
        id.to_string(),
        task.title, 
        task.completed,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            }).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({
            "success": true, 
            "data": result,
            "message": "Create task success"
        }).to_string(),
    ))
}


async fn get_tasks (
    State(pool): State<SqlitePool> 
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks"
    ).fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "success": false,
                "data": null,
                "message": e.to_string()
            }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({
            "success": true, 
            "data": result,
            "message": "Get tasks success"
        }).to_string(),
    ))
}