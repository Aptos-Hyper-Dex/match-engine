use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            message: None,
        }
    }

    pub fn with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: usize, page: u32, page_size: u32) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as u32;
        Self {
            data,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    DatabaseError(String),
    RedisError(String),
    OrderBookError(String),
    ValidationError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            ApiError::RedisError(msg) => write!(f, "Redis Error: {}", msg),
            ApiError::OrderBookError(msg) => write!(f, "Order Book Error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(_) => HttpResponse::NotFound().json(ApiResponse::<()>::error(self.to_string())),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(self.to_string())),
            ApiError::ValidationError(_) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(self.to_string())),
            _ => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(self.to_string())),
        }
    }
}
