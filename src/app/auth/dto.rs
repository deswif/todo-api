use serde::{Deserialize, Serialize};
use crate::app::user::dto::UserResponse;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: TokenDetails,
    pub user: UserResponse,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub token: TokenDetails,
    pub user: UserResponse,
}

#[derive(Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: String,
}
