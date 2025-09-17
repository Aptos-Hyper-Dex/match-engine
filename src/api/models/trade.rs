use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub price: u64,
    pub quantity: u64,
    pub side: TradeSide,
    pub taker_order_id: Uuid,
    pub maker_order_id: Uuid,
    pub taker_user_id: Uuid,
    pub maker_user_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResponse {
    pub trades: Vec<Trade>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeStats {
    pub symbol: String,
    pub total_volume: u64,
    pub total_trades: u64,
    pub avg_price: f64,
    pub high_price: u64,
    pub low_price: u64,
    pub timestamp: DateTime<Utc>,
}

impl From<pricelevel::Side> for TradeSide {
    fn from(side: pricelevel::Side) -> Self {
        match side {
            pricelevel::Side::Buy => TradeSide::Buy,
            pricelevel::Side::Sell => TradeSide::Sell,
        }
    }
}
