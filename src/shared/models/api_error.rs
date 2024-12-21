use actix_web::body::BoxBody;
use actix_web::error::{ResponseError};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Display)]
pub enum ServerError {
    UnknownError(String),
    NoBearerToken,
    InvalidTokenFormat,
    ParseError(String),
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::NoBearerToken => StatusCode::UNAUTHORIZED,
            ServerError::InvalidTokenFormat => StatusCode::FORBIDDEN,
            ServerError::ParseError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = match self {
            ServerError::UnknownError(msg) => ErrorResponse {
                error_code: "unknown".to_string(),
                error: None,
                message: Some(msg.to_string()),
            },
            ServerError::NoBearerToken => ErrorResponse {
                error_code: "no-bearer-token".to_string(),
                error: Some("No bearer token".to_string()),
                message: Some("Bearer token is required in header to access this resource".to_string()),
            },
            ServerError::InvalidTokenFormat => ErrorResponse {
                error_code: "invalid-token-format".to_string(),
                error: Some("Invalid token format".to_string()),
                message: Some("Provided bearer token cannot be decoded".to_string()),
            },
            ServerError::ParseError(msg) => ErrorResponse {
                error_code: "parse-error".to_string(),
                error: Some(msg.to_string()),
                message: None,
            }
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}
