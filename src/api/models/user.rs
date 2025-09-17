use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalance {
    pub user_id: Uuid,
    pub asset: String,
    pub available: u64,
    pub locked: u64,
    pub total: u64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrdersResponse {
    pub orders: Vec<crate::api::models::order::OrderResponse>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
}
