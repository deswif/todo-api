use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::services::models::todo::Todo;

#[derive(Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub is_complete: bool,
    pub created_at: DateTime<Utc>,
}

impl TodoResponse {
    pub fn new(todo: &Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title.clone(),
            is_complete: todo.is_complete,
            created_at: DateTime::from(todo.created_at),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompleteTodoRequest {
    pub is_complete: Option<bool>,
}
