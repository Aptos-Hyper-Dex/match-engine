use redis::{aio::Connection, AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct RedisClient {
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookCache {
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
pub struct PriceLevelCache {
    pub price: u64,
    pub visible_quantity: u64,
    pub hidden_quantity: u64,
    pub order_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCache {
    pub id: String,
    pub symbol: String,
    pub price: u64,
    pub quantity: u64,
    pub side: String,
    pub timestamp: DateTime<Utc>,
}

impl RedisClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let client = Client::open(redis_url)?;
        // Lazily create connections per operation to avoid needing ConnectionManager in this redis version
        // Validate connectivity once
        let mut conn = client.get_async_connection().await?;
        let _: () = redis::cmd("PING").query_async(&mut conn).await?;

        Ok(RedisClient { client })
    }

    // OrderBook cache operations
    pub async fn cache_orderbook(&self, symbol: &str, orderbook: &OrderBookCache) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("orderbook:{}", symbol);
        let value = serde_json::to_string(orderbook)?;
        
        let mut conn = self.client.get_async_connection().await?;
        let _: () = conn.set_ex(key, value, 60).await?; // Cache for 60 seconds
        Ok(())
    }

    pub async fn get_orderbook(&self, symbol: &str) -> Result<Option<OrderBookCache>, Box<dyn std::error::Error>> {
        let key = format!("orderbook:{}", symbol);
        let mut conn: Connection = self.client.get_async_connection().await?;
        
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let orderbook: OrderBookCache = serde_json::from_str(&v)?;
                Ok(Some(orderbook))
            }
            None => Ok(None),
        }
    }

    // Price levels cache operations
    pub async fn cache_price_levels(&self, symbol: &str, side: &str, levels: &[PriceLevelCache]) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("price_levels:{}:{}", symbol, side);
        let value = serde_json::to_string(levels)?;
        
        let mut conn = self.client.get_async_connection().await?;
        let _: () = conn.set_ex(key, value, 30).await?; // Cache for 30 seconds
        Ok(())
    }

    pub async fn get_price_levels(&self, symbol: &str, side: &str) -> Result<Option<Vec<PriceLevelCache>>, Box<dyn std::error::Error>> {
        let key = format!("price_levels:{}:{}", symbol, side);
        let mut conn = self.client.get_async_connection().await?;
        
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let levels: Vec<PriceLevelCache> = serde_json::from_str(&v)?;
                Ok(Some(levels))
            }
            None => Ok(None),
        }
    }

    // Trade cache operations
    pub async fn cache_recent_trades(&self, symbol: &str, trades: &[TradeCache]) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("trades:{}", symbol);
        let value = serde_json::to_string(trades)?;
        
        let mut conn = self.client.get_async_connection().await?;
        let _: () = conn.set_ex(key, value, 300).await?; // Cache for 5 minutes
        Ok(())
    }

    pub async fn get_recent_trades(&self, symbol: &str) -> Result<Option<Vec<TradeCache>>, Box<dyn std::error::Error>> {
        let key = format!("trades:{}", symbol);
        let mut conn = self.client.get_async_connection().await?;
        
        let value: Option<String> = conn.get(key).await?;
        match value {
            Some(v) => {
                let trades: Vec<TradeCache> = serde_json::from_str(&v)?;
                Ok(Some(trades))
            }
            None => Ok(None),
        }
    }

    // Market data cache operations
    pub async fn cache_market_data(&self, symbol: &str, data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("market_data:{}", symbol);
        let mut conn = self.client.get_async_connection().await?;
        
        for (field, value) in data {
            let _: () = conn.hset(key.clone(), field, value).await?;
        }
        let _: () = conn.expire(key, 60).await?; // Cache for 60 seconds
        Ok(())
    }

    pub async fn get_market_data(&self, symbol: &str) -> Result<Option<HashMap<String, String>>, Box<dyn std::error::Error>> {
        let key = format!("market_data:{}", symbol);
        let mut conn = self.client.get_async_connection().await?;
        
        let data: HashMap<String, String> = conn.hgetall(key).await?;
        if data.is_empty() { return Ok(None); }
        Ok(Some(data))
    }

    // Volume statistics cache
    pub async fn cache_volume_stats(&self, symbol: &str, stats: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("volume_stats:{}", symbol);
        let mut conn = self.client.get_async_connection().await?;
        
        for (field, value) in stats {
            let _: () = conn.hset(key.clone(), field, value).await?;
        }
        let _: () = conn.expire(key, 300).await?; // Cache for 5 minutes
        Ok(())
    }

    pub async fn get_volume_stats(&self, symbol: &str) -> Result<Option<HashMap<String, String>>, Box<dyn std::error::Error>> {
        let key = format!("volume_stats:{}", symbol);
        let mut conn = self.client.get_async_connection().await?;
        
        let stats: HashMap<String, String> = conn.hgetall(key).await?;
        if stats.is_empty() { return Ok(None); }
        Ok(Some(stats))
    }

    // Real-time trade streaming
    pub async fn publish_trade(&self, symbol: &str, trade: &TradeCache) -> Result<(), Box<dyn std::error::Error>> {
        let channel = format!("trades:{}", symbol);
        let message = serde_json::to_string(trade)?;
        
        let mut conn = self.client.get_async_connection().await?;
        let _: () = conn.publish(channel, message).await?;
        Ok(())
    }

    // Order book updates streaming
    pub async fn publish_orderbook_update(&self, symbol: &str, update: &OrderBookCache) -> Result<(), Box<dyn std::error::Error>> {
        let channel = format!("orderbook:{}", symbol);
        let message = serde_json::to_string(update)?;
        
        let mut conn = self.client.get_async_connection().await?;
        let _: () = conn.publish(channel, message).await?;
        Ok(())
    }

    // Clear cache for a symbol
    pub async fn clear_symbol_cache(&self, symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_async_connection().await?;
        
        let patterns = vec![
            format!("orderbook:{}", symbol),
            format!("price_levels:{}:*", symbol),
            format!("trades:{}", symbol),
            format!("market_data:{}", symbol),
            format!("volume_stats:{}", symbol),
        ];

        for pattern in patterns {
            let keys: Vec<String> = conn.keys(pattern).await?;
            if !keys.is_empty() {
                let _: () = conn.del(keys).await?;
            }
        }

        Ok(())
    }
}
