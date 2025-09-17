use actix_web::{web, HttpResponse, Result};
use dashmap::DashMap;
use crate::OrderBook;
use pricelevel::{OrderId, OrderUpdate, Side, TimeInForce};
use std::sync::Arc;

use crate::api::{
    database::Database,
    models::{order::*, response::ApiResponse},
    redis::RedisClient,
};
use crate::orderbook::modifications::OrderQuantity;

#[derive(serde::Deserialize)]
pub struct PathOrderId { pub order_id: String }

#[derive(serde::Deserialize)]
pub struct PathUserId { pub user_id: String }

pub async fn create_order(
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    db: web::Data<Database>,
    _redis: web::Data<RedisClient>,
    payload: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse> {
    let req = payload.into_inner();
    let Some(orderbook) = orderbooks.get(&req.symbol) else {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(format!(
            "Order book for symbol {} not found", req.symbol
        ))));
    };

    let id = OrderId(req.user_id); // not ideal; generate real order id
    let side: Side = req.side.clone().into();
    let tif: TimeInForce = req.time_in_force.clone().into();

    match req.order_type {
        OrderType::Market => {
            let qty = req.quantity;
            match orderbook.submit_market_order(id, qty, side) {
                Ok(result) => {
                    let body = serde_json::json!({
                        "executed": result.executed_quantity(),
                        "remaining": result.remaining_quantity,
                        "complete": result.is_complete,
                        "transactions": result.transactions.transactions.len()
                    });
                    Ok(HttpResponse::Ok().json(ApiResponse::success(body)))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
            }
        }
        OrderType::Limit | OrderType::ImmediateOrCancel | OrderType::FillOrKill => {
            let price = req.price.ok_or_else(|| actix_web::error::ErrorBadRequest("price is required for limit/IOC/FOK"))?;
            match orderbook.add_limit_order(id, price, req.quantity, side, tif) {
                Ok(order_arc) => {
                    // Persist order (best-effort)
                    let _ = persist_order(&db, &req, order_arc.id().0.to_string(), price, req.quantity).await;
                    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                        "order_id": order_arc.id(),
                        "status": "PENDING"
                    }))))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
            }
        }
        OrderType::PostOnly => {
            let price = req.price.ok_or_else(|| actix_web::error::ErrorBadRequest("price is required for post-only"))?;
            match orderbook.add_post_only_order(id, price, req.quantity, side, tif) {
                Ok(order_arc) => {
                    let _ = persist_order(&db, &req, order_arc.id().0.to_string(), price, req.quantity).await;
                    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                        "order_id": order_arc.id(),
                        "status": "PENDING"
                    }))))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
            }
        }
        OrderType::Iceberg => {
            let price = req.price.ok_or_else(|| actix_web::error::ErrorBadRequest("price is required for iceberg"))?;
            let vis = req.visible_quantity.ok_or_else(|| actix_web::error::ErrorBadRequest("visible_quantity required"))?;
            let hid = req.hidden_quantity.ok_or_else(|| actix_web::error::ErrorBadRequest("hidden_quantity required"))?;
            match orderbook.add_iceberg_order(id, price, vis, hid, side, tif) {
                Ok(order_arc) => {
                    let _ = persist_order(&db, &req, order_arc.id().0.to_string(), price, vis + hid).await;
                    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                        "order_id": order_arc.id(),
                        "status": "PENDING"
                    }))))
                }
                Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("unsupported order type for this endpoint".to_string()))),
    }
}

pub async fn get_order(
    path: web::Path<PathOrderId>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
) -> Result<HttpResponse> {
    let _order_id = &path.order_id;
    // Without symbol lookup, we cannot efficiently find; iterate
    for item in orderbooks.iter() {
        if let Ok(uuid) = uuid::Uuid::parse_str(&_order_id) {
            let id = OrderId(uuid);
            if let Some(order) = item.value().get_order(id) {
                let resp = serde_json::json!({
                    "order_id": order.id(),
                    "symbol": item.key().clone(),
                    "price": order.price(),
                    "quantity": order.quantity(),
                    "side": format!("{:?}", order.side()),
                    "time_in_force": format!("{:?}", order.time_in_force()),
                });
                return Ok(HttpResponse::Ok().json(ApiResponse::success(resp)));
            }
        }
    }
    Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("order not found".to_string())))
}

pub async fn update_order(
    path: web::Path<PathOrderId>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
    payload: web::Json<UpdateOrderRequest>,
) -> Result<HttpResponse> {
    let order_uuid = uuid::Uuid::parse_str(&path.order_id).map_err(|_| actix_web::error::ErrorBadRequest("invalid order_id"))?;
    let id = OrderId(order_uuid);
    let req = payload.into_inner();

    for item in orderbooks.iter() {
        let ob = item.value();
        let result = if let (Some(price), Some(qty)) = (req.price, req.quantity) {
            ob.update_order(OrderUpdate::UpdatePriceAndQuantity { order_id: id, new_price: price, new_quantity: qty })
        } else if let Some(price) = req.price {
            ob.update_order(OrderUpdate::UpdatePrice { order_id: id, new_price: price })
        } else if let Some(qty) = req.quantity {
            ob.update_order(OrderUpdate::UpdateQuantity { order_id: id, new_quantity: qty })
        } else {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("nothing to update".to_string())));
        };

        match result {
            Ok(_) => return Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"updated": true})))) ,
            Err(_) => continue,
        }
    }

    Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("order not found".to_string())))
}

pub async fn cancel_order(
    path: web::Path<PathOrderId>,
    orderbooks: web::Data<Arc<DashMap<String, Arc<OrderBook>>>>,
) -> Result<HttpResponse> {
    let order_uuid = uuid::Uuid::parse_str(&path.order_id).map_err(|_| actix_web::error::ErrorBadRequest("invalid order_id"))?;
    let id = OrderId(order_uuid);
    for item in orderbooks.iter() {
        match item.value().cancel_order(id) {
            Ok(Some(_)) => return Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"cancelled": true})))) ,
            Ok(None) => continue,
            Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
        }
    }
    Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("order not found".to_string())))
}

pub async fn get_user_orders(
    _path: web::Path<PathUserId>,
    _db: web::Data<Database>,
) -> Result<HttpResponse> {
    // TODO: implement real DB query; return empty for now
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "orders": [],
        "total": 0
    }))))
}

async fn persist_order(
    db: &Database,
    req: &CreateOrderRequest,
    order_id: String,
    price: u64,
    total_qty: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = db.pool.get().await?;
    let _ = client.execute(
        "INSERT INTO orders (id, symbol, side, order_type, quantity, price, time_in_force, status, user_id, remaining_quantity, visible_quantity, hidden_quantity) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
        &[
            &uuid::Uuid::parse_str(&order_id)?,
            &req.symbol,
            &format!("{:?}", req.side),
            &format!("{:?}", req.order_type),
            &(total_qty as i64),
            &(price as i64),
            &format!("{:?}", req.time_in_force),
            &"PENDING",
            &req.user_id,
            &(total_qty as i64),
            &req.visible_quantity.map(|v| v as i64),
            &req.hidden_quantity.map(|v| v as i64),
        ],
    ).await?;
    Ok(())
}


