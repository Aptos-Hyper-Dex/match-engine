use actix_web::{web, HttpResponse, Result};
use dashmap::DashMap;
use std::sync::Arc;
use crate::OrderBook;
use crate::api::{
    models::{orderbook::BestPricesResponse, response::ApiResponse, trade::{TradeResponse, VolumeStats}},
    redis::RedisClient,
};

#[derive(serde::Deserialize)]
pub struct Pagination { pub page: Option<u32>, pub page_size: Option<u32> }

pub async fn get_best_prices(
    path: web::Path<String>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    if let Some(orderbook) = orderbooks.get(&symbol) {
        let response = BestPricesResponse {
            symbol: symbol.clone(),
            best_bid: orderbook.best_bid(),
            best_ask: orderbook.best_ask(),
            spread: orderbook.spread(),
            mid_price: orderbook.mid_price(),
            timestamp: chrono::Utc::now(),
        };
        Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            format!("Order book for symbol {} not found", symbol)
        )))
    }
}

pub async fn get_recent_trades(
    path: web::Path<String>,
    _pagination: web::Query<Pagination>,
    redis: web::Data<RedisClient>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    if let Ok(Some(trades)) = redis.get_recent_trades(&symbol).await {
        let response = TradeResponse { trades: trades.into_iter().map(|t| crate::api::models::trade::Trade {
            id: uuid::Uuid::parse_str(&t.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            symbol: t.symbol,
            price: t.price,
            quantity: t.quantity,
            side: match t.side.as_str() { "Buy" => crate::api::models::trade::TradeSide::Buy, _ => crate::api::models::trade::TradeSide::Sell },
            taker_order_id: uuid::Uuid::new_v4(),
            maker_order_id: uuid::Uuid::new_v4(),
            taker_user_id: uuid::Uuid::new_v4(),
            maker_user_id: uuid::Uuid::new_v4(),
            timestamp: t.timestamp,
        }).collect(), total: 0, page: 1, page_size: 50 };
        return Ok(HttpResponse::Ok().json(ApiResponse::success(response)));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(TradeResponse { trades: vec![], total: 0, page: 1, page_size: 50 })))
}

pub async fn get_volume_stats(
    path: web::Path<String>,
    redis: web::Data<RedisClient>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    if let Ok(Some(stats)) = redis.get_volume_stats(&symbol).await {
        let response = VolumeStats {
            symbol: symbol.clone(),
            total_volume: stats.get("total_volume").and_then(|v| v.parse().ok()).unwrap_or(0),
            total_trades: stats.get("total_trades").and_then(|v| v.parse().ok()).unwrap_or(0),
            avg_price: stats.get("avg_price").and_then(|v| v.parse().ok()).unwrap_or(0.0),
            high_price: stats.get("high_price").and_then(|v| v.parse().ok()).unwrap_or(0),
            low_price: stats.get("low_price").and_then(|v| v.parse().ok()).unwrap_or(0),
            timestamp: chrono::Utc::now(),
        };
        Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::success(VolumeStats {
            symbol,
            total_volume: 0,
            total_trades: 0,
            avg_price: 0.0,
            high_price: 0,
            low_price: 0,
            timestamp: chrono::Utc::now(),
        })))
    }
}

