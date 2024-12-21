use std::time::SystemTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::todo;

#[derive(Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = todo)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub is_complete: bool,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct CreateTodoDTO {
    pub user_id: i32,
    pub title: String,
}
