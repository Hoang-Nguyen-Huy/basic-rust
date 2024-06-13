use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub completed: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>
}