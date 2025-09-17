use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: u64,
    pub price: Option<u64>,
    pub time_in_force: TimeInForce,
    pub user_id: Uuid,
    // For iceberg orders
    pub visible_quantity: Option<u64>,
    pub hidden_quantity: Option<u64>,
    // For post-only orders
    pub post_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub quantity: Option<u64>,
    pub price: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: u64,
    pub price: Option<u64>,
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub filled_quantity: u64,
    pub remaining_quantity: u64,
    // For iceberg orders
    pub visible_quantity: Option<u64>,
    pub hidden_quantity: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Iceberg,
    PostOnly,
    FillOrKill,
    ImmediateOrCancel,
    GoodTillDate,
    TrailingStop,
    Pegged,
    MarketToLimit,
    Reserve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    Gtc, // Good Till Cancel
    Ioc, // Immediate Or Cancel
    Fok, // Fill Or Kill
    Day, // Good Till End of Day
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

impl From<pricelevel::Side> for OrderSide {
    fn from(side: pricelevel::Side) -> Self {
        match side {
            pricelevel::Side::Buy => OrderSide::Buy,
            pricelevel::Side::Sell => OrderSide::Sell,
        }
    }
}

impl From<OrderSide> for pricelevel::Side {
    fn from(side: OrderSide) -> Self {
        match side {
            OrderSide::Buy => pricelevel::Side::Buy,
            OrderSide::Sell => pricelevel::Side::Sell,
        }
    }
}

impl From<pricelevel::TimeInForce> for TimeInForce {
    fn from(tif: pricelevel::TimeInForce) -> Self {
        match tif {
            pricelevel::TimeInForce::Gtc => TimeInForce::Gtc,
            pricelevel::TimeInForce::Ioc => TimeInForce::Ioc,
            pricelevel::TimeInForce::Fok => TimeInForce::Fok,
            pricelevel::TimeInForce::Day => TimeInForce::Day,
            pricelevel::TimeInForce::Gtd(_) => TimeInForce::Gtc, // Map GTD to GTC for now
        }
    }
}

impl From<TimeInForce> for pricelevel::TimeInForce {
    fn from(tif: TimeInForce) -> Self {
        match tif {
            TimeInForce::Gtc => pricelevel::TimeInForce::Gtc,
            TimeInForce::Ioc => pricelevel::TimeInForce::Ioc,
            TimeInForce::Fok => pricelevel::TimeInForce::Fok,
            TimeInForce::Day => pricelevel::TimeInForce::Day,
        }
    }
}
