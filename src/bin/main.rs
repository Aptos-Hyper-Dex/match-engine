use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use orderbook_rs::OrderBook;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber;

use orderbook_rs::api as api;
use api::{
    database::Database,
    handlers::{
        order_handlers, orderbook_handlers, query_handlers,
    },
    middleware::error_handlers,
    redis::RedisClient,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting OrderBook API Server...");

    // Initialize database connection and schema
    let database = Database::new().await.expect("Failed to connect to database");
    let _ = database.init_schema().await;
    info!("Database connection established");

    // Initialize Redis connection
    let redis_client = RedisClient::new().await.expect("Failed to connect to Redis");
    info!("Redis connection established");

    // Create shared order book instances for different symbols
    let orderbooks = Arc::new(dashmap::DashMap::new());
    
    // Initialize order books for major trading pairs
    let symbols = vec!["BTC/USD", "ETH/USD", "LTC/USD"];
    for symbol in symbols {
        let orderbook = Arc::new(OrderBook::new(symbol));
        orderbooks.insert(symbol.to_string(), orderbook);
        info!("Initialized order book for {}", symbol);
    }

    // Start HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(orderbooks.clone()))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/orderbook")
                            .route("", web::get().to(orderbook_handlers::get_orderbooks))
                            .route("/{symbol}", web::get().to(orderbook_handlers::get_orderbook))
                            .route("/{symbol}/snapshot", web::get().to(orderbook_handlers::get_snapshot))
                            .route("/{symbol}/depth", web::get().to(orderbook_handlers::get_depth))
                    )
                    .service(
                        web::scope("/orders")
                            .route("", web::post().to(order_handlers::create_order))
                            .route("/{order_id}", web::get().to(order_handlers::get_order))
                            .route("/{order_id}", web::put().to(order_handlers::update_order))
                            .route("/{order_id}", web::delete().to(order_handlers::cancel_order))
                            .route("/user/{user_id}", web::get().to(order_handlers::get_user_orders))
                    )
                    .service(
                        web::scope("/query")
                            .route("/best-prices/{symbol}", web::get().to(query_handlers::get_best_prices))
                            .route("/trades/{symbol}", web::get().to(query_handlers::get_recent_trades))
                            .route("/volume/{symbol}", web::get().to(query_handlers::get_volume_stats))
                    )
            )
            .default_service(web::route().to(error_handlers::not_found))
    })
    .bind("0.0.0.0:8080")?
    .run();

    info!("OrderBook API Server running on http://0.0.0.0:8080");
    server.await
}
