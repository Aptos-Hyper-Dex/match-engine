use actix_web::{web, HttpResponse, Result};
use dashmap::DashMap;
use std::sync::Arc;
use crate::OrderBook;
use crate::api::{
    models::{orderbook::*, response::*},
    redis::RedisClient,
};

pub async fn get_orderbooks(
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    redis: web::Data<RedisClient>,
) -> Result<HttpResponse> {
    let mut orderbook_list = Vec::new();

    for item in orderbooks.iter() {
        let symbol = item.key();
        let orderbook = item.value();

        // Try to get from cache first
        if let Ok(Some(cached)) = redis.get_orderbook(symbol).await {
            orderbook_list.push(OrderBookResponse {
                symbol: symbol.clone(),
                best_bid: cached.best_bid,
                best_ask: cached.best_ask,
                spread: cached.spread,
                mid_price: cached.mid_price,
                last_trade_price: cached.last_trade_price,
                total_orders: cached.total_orders,
                bid_levels: cached.bid_levels,
                ask_levels: cached.ask_levels,
                total_bid_quantity: cached.total_bid_quantity,
                total_ask_quantity: cached.total_ask_quantity,
                timestamp: cached.timestamp,
            });
        } else {
            // Build response from orderbook
            let (bid_volumes, ask_volumes) = orderbook.get_volume_by_price();
            let response = OrderBookResponse {
                symbol: symbol.clone(),
                best_bid: orderbook.best_bid(),
                best_ask: orderbook.best_ask(),
                spread: orderbook.spread(),
                mid_price: orderbook.mid_price(),
                last_trade_price: orderbook.last_trade_price(),
                total_orders: orderbook.get_all_orders().len(),
                bid_levels: bid_volumes.len(),
                ask_levels: ask_volumes.len(),
                total_bid_quantity: bid_volumes.values().sum(),
                total_ask_quantity: ask_volumes.values().sum(),
                timestamp: chrono::Utc::now(),
            };

            // Cache the response
            let cache_data = crate::api::redis::OrderBookCache {
                symbol: symbol.clone(),
                best_bid: response.best_bid,
                best_ask: response.best_ask,
                spread: response.spread,
                mid_price: response.mid_price,
                last_trade_price: response.last_trade_price,
                total_orders: response.total_orders,
                bid_levels: response.bid_levels,
                ask_levels: response.ask_levels,
                total_bid_quantity: response.total_bid_quantity,
                total_ask_quantity: response.total_ask_quantity,
                timestamp: response.timestamp,
            };

            let _ = redis.cache_orderbook(symbol, &cache_data).await;
            orderbook_list.push(response);
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(orderbook_list)))
}

pub async fn get_orderbook(
    path: web::Path<String>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    redis: web::Data<RedisClient>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();

    if let Some(orderbook) = orderbooks.get(&symbol) {
        // Try to get from cache first
        if let Ok(Some(cached)) = redis.get_orderbook(&symbol).await {
            let response = OrderBookResponse {
                symbol: symbol.clone(),
                best_bid: cached.best_bid,
                best_ask: cached.best_ask,
                spread: cached.spread,
                mid_price: cached.mid_price,
                last_trade_price: cached.last_trade_price,
                total_orders: cached.total_orders,
                bid_levels: cached.bid_levels,
                ask_levels: cached.ask_levels,
                total_bid_quantity: cached.total_bid_quantity,
                total_ask_quantity: cached.total_ask_quantity,
                timestamp: cached.timestamp,
            };
            return Ok(HttpResponse::Ok().json(ApiResponse::success(response)));
        }

        // Build response from orderbook
        let (bid_volumes, ask_volumes) = orderbook.get_volume_by_price();
        let response = OrderBookResponse {
            symbol: symbol.clone(),
            best_bid: orderbook.best_bid(),
            best_ask: orderbook.best_ask(),
            spread: orderbook.spread(),
            mid_price: orderbook.mid_price(),
            last_trade_price: orderbook.last_trade_price(),
            total_orders: orderbook.get_all_orders().len(),
            bid_levels: bid_volumes.len(),
            ask_levels: ask_volumes.len(),
            total_bid_quantity: bid_volumes.values().sum(),
            total_ask_quantity: ask_volumes.values().sum(),
            timestamp: chrono::Utc::now(),
        };

        // Cache the response
        let cache_data = crate::api::redis::OrderBookCache {
            symbol: symbol.clone(),
            best_bid: response.best_bid,
            best_ask: response.best_ask,
            spread: response.spread,
            mid_price: response.mid_price,
            last_trade_price: response.last_trade_price,
            total_orders: response.total_orders,
            bid_levels: response.bid_levels,
            ask_levels: response.ask_levels,
            total_bid_quantity: response.total_bid_quantity,
            total_ask_quantity: response.total_ask_quantity,
            timestamp: response.timestamp,
        };

        let _ = redis.cache_orderbook(&symbol, &cache_data).await;

        Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            format!("Order book for symbol {} not found", symbol)
        )))
    }
}

#[derive(serde::Deserialize)]
pub struct DepthQuery { pub depth: Option<usize> }

pub async fn get_snapshot(
    path: web::Path<String>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    query: web::Query<DepthQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let depth = query.depth.unwrap_or(10);

    if let Some(orderbook) = orderbooks.get(&symbol) {
        let snapshot = orderbook.create_snapshot(depth);
        
        let response = OrderBookSnapshot {
            symbol: snapshot.symbol,
            timestamp: chrono::DateTime::from_timestamp_millis(snapshot.timestamp as i64)
                .unwrap_or_else(|| chrono::Utc::now()),
            bids: snapshot.bids.into_iter().map(|level| PriceLevel {
                price: level.price,
                visible_quantity: level.visible_quantity,
                hidden_quantity: level.hidden_quantity,
                order_count: level.order_count,
            }).collect(),
            asks: snapshot.asks.into_iter().map(|level| PriceLevel {
                price: level.price,
                visible_quantity: level.visible_quantity,
                hidden_quantity: level.hidden_quantity,
                order_count: level.order_count,
            }).collect(),
        };

        Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            format!("Order book for symbol {} not found", symbol)
        )))
    }
}

pub async fn get_depth(
    path: web::Path<String>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    query: web::Query<DepthQuery>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let depth = query.depth.unwrap_or(10);

    if let Some(orderbook) = orderbooks.get(&symbol) {
        let snapshot = orderbook.create_snapshot(depth);
        
        let response = DepthResponse {
            symbol: snapshot.symbol,
            bids: snapshot.bids.into_iter().map(|level| PriceLevel {
                price: level.price,
                visible_quantity: level.visible_quantity,
                hidden_quantity: level.hidden_quantity,
                order_count: level.order_count,
            }).collect(),
            asks: snapshot.asks.into_iter().map(|level| PriceLevel {
                price: level.price,
                visible_quantity: level.visible_quantity,
                hidden_quantity: level.hidden_quantity,
                order_count: level.order_count,
            }).collect(),
            timestamp: chrono::DateTime::from_timestamp_millis(snapshot.timestamp as i64)
                .unwrap_or_else(|| chrono::Utc::now()),
        };

        Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(
            format!("Order book for symbol {} not found", symbol)
        )))
    }
}
