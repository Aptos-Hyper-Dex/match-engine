use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use std::env;
use url;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/orderbook".to_string());

        let mut cfg = Config::new();
        // Parse connection string into individual components
        if let Ok(parsed) = url::Url::parse(&database_url) {
            if let Some(host) = parsed.host_str() {
                cfg.host = Some(host.to_string());
            }
            if let Some(port) = parsed.port() {
                cfg.port = Some(port);
            }
            if !parsed.username().is_empty() {
                cfg.user = Some(parsed.username().to_string());
            }
            if let Some(password) = parsed.password() {
                cfg.password = Some(password.to_string());
            }
            if let Some(path) = parsed.path().strip_prefix('/') {
                cfg.dbname = Some(path.to_string());
            }
        }
        cfg.pool = Some(deadpool_postgres::PoolConfig::new(10));

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

        // Test the connection
        let client = pool.get().await?;
        client.execute("SELECT 1", &[]).await?;

        Ok(Database { pool })
    }

    pub async fn init_schema(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Create users table
        client.execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(100) UNIQUE NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                is_active BOOLEAN DEFAULT TRUE
            )
            "#,
            &[],
        ).await?;

        // Create user_balances table
        client.execute(
            r#"
            CREATE TABLE IF NOT EXISTS user_balances (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL REFERENCES users(id),
                asset VARCHAR(10) NOT NULL,
                available BIGINT NOT NULL DEFAULT 0,
                locked BIGINT NOT NULL DEFAULT 0,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                UNIQUE(user_id, asset)
            )
            "#,
            &[],
        ).await?;

        // Create orders table
        client.execute(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id UUID PRIMARY KEY,
                symbol VARCHAR(20) NOT NULL,
                side VARCHAR(4) NOT NULL,
                order_type VARCHAR(20) NOT NULL,
                quantity BIGINT NOT NULL,
                price BIGINT,
                time_in_force VARCHAR(3) NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
                user_id UUID NOT NULL REFERENCES users(id),
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                filled_quantity BIGINT NOT NULL DEFAULT 0,
                remaining_quantity BIGINT NOT NULL,
                visible_quantity BIGINT,
                hidden_quantity BIGINT
            )
            "#,
            &[],
        ).await?;

        // Create trades table
        client.execute(
            r#"
            CREATE TABLE IF NOT EXISTS trades (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                symbol VARCHAR(20) NOT NULL,
                price BIGINT NOT NULL,
                quantity BIGINT NOT NULL,
                side VARCHAR(4) NOT NULL,
                taker_order_id UUID NOT NULL,
                maker_order_id UUID NOT NULL,
                taker_user_id UUID NOT NULL REFERENCES users(id),
                maker_user_id UUID NOT NULL REFERENCES users(id),
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
            &[],
        ).await?;

        // Create indexes
        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id)",
            &[],
        ).await?;

        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_symbol ON orders(symbol)",
            &[],
        ).await?;

        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)",
            &[],
        ).await?;

        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol)",
            &[],
        ).await?;

        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_trades_created_at ON trades(created_at)",
            &[],
        ).await?;

        Ok(())
    }
}
