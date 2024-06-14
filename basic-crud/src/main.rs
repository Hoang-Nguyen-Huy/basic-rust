use axum:: {
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
// use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;
use std::net::SocketAddr;

mod models;
use models::{CreateTask, Task, UpdateTask};

const SCHEMA: &str = include_str!("../schema.sql");

#[tokio::main]
async fn main () {
    dotenvy::dotenv().expect("Unable to access .env file");
    let data_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    let pool = SqlitePool::connect(&data_url).await.unwrap();

    sqlx::query(SCHEMA).execute(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(hello))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task_by_id).delete(delete_task).put(update_task))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}" , addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await.unwrap();
}

async fn hello () -> &'static str {
    "Hello this is basic CRUD with Rust"
}

async fn create_task (
    State(pool): State<SqlitePool>,
    Json(task): Json<CreateTask>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"INSERT INTO tasks(id, title, completed) VALUES(?1, ?2, ?3)"#,
        id,
        task.title, 
        task.completed,
    )
    .execute(&pool)
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
            "data": null,
            "message": "Create task success"
        }).to_string(),
    ))
}


async fn get_tasks (
    State(pool): State<SqlitePool> 
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query_as!(
        Task,
        r#"SELECT id as "id?", title, completed FROM tasks"#
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

async fn get_task_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query_as!(
            Task,
            r#"SELECT id as "id?", title, completed FROM tasks WHERE id = ?1"#,
            id
        ).fetch_one(&pool)
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

async fn delete_task (
    State(pool): State<SqlitePool>,
    Path(id): Path<String>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        r#"DELETE FROM tasks WHERE id = ?1"#,
        id
    ).execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "success": false,
                "data": null,
                "message": e.to_string(),
            }).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": null,
            "message": "Delete task successful"
        }).to_string(),
    ))
}

async fn update_task (
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(task): Json<UpdateTask>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut query = "UPDATE tasks SET ".to_owned();

    let mut i = 2;

    if task.title.is_some() {
        query.push_str(&format!("title = ?{i}"));
        i = i + 1;
    }

    if task.completed.is_some() {
        query.push_str(&format!(", completed = ?{i}"));
    }

    query.push_str(&format!(" WHERE id = ?1"));

    let mut start = sqlx::query(&query).bind(id);

    if task.title.is_some() {
        start = start.bind(task.title);
    }

    if task.completed.is_some() {
        start = start.bind(task.completed);
    }

    start.execute(&pool).await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({
                    "success": false, 
                    "data": null,
                    "message": e.to_string(),
                }).to_string(),
            )
        })?;
   
    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": null,
            "message": "Update task successful"
        }).to_string(),
    ))
}