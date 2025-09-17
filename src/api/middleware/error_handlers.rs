use actix_web::{HttpResponse, Result};
use crate::api::models::response::ApiResponse;

pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Route not found".to_string())))
}

