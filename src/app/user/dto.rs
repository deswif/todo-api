use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::services::models::user::User;

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl UserResponse {
    pub fn new(user: &User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email.clone(),
            created_at: DateTime::<Utc>::from(user.created_at.clone()),
        }
    }
}