use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookResponse {
    pub symbol: String,
    pub best_bid: Option<u64>,
    pub best_ask: Option<u64>,
    pub spread: Option<u64>,
    pub mid_price: Option<f64>,
    pub last_trade_price: Option<u64>,
    pub total_orders: usize,
    pub bid_levels: usize,
    pub ask_levels: usize,
    pub total_bid_quantity: u64,
    pub total_ask_quantity: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: u64,
    pub visible_quantity: u64,
    pub hidden_quantity: u64,
    pub order_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthResponse {
    pub symbol: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPricesResponse {
    pub symbol: String,
    pub best_bid: Option<u64>,
    pub best_ask: Option<u64>,
    pub spread: Option<u64>,
    pub mid_price: Option<f64>,
    pub timestamp: DateTime<Utc>,
}
