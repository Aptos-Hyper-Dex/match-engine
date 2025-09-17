# OrderBook API 接口文档

## 🚀 快速开始

### 基础信息
- **Base URL**: `http://localhost:8080`
- **API版本**: `/api/v1`
- **内容类型**: `application/json`

### 服务启动
```bash
cargo run --bin orderbook-api
```

## 📋 接口概览

### 订单管理
| 方法 | 路径 | 描述 |
|------|------|------|
| POST | `/api/v1/orders` | 创建新订单 |
| GET | `/api/v1/orders/{order_id}` | 查询订单详情 |
| PUT | `/api/v1/orders/{order_id}` | 更新订单 |
| DELETE | `/api/v1/orders/{order_id}` | 取消订单 |
| GET | `/api/v1/orders/user/{user_id}` | 获取用户订单列表 |

### 订单簿查询
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/api/v1/orderbook` | 获取所有订单簿 |
| GET | `/api/v1/orderbook/{symbol}` | 获取指定交易对订单簿 |
| GET | `/api/v1/orderbook/{symbol}/snapshot` | 获取订单簿快照 |
| GET | `/api/v1/orderbook/{symbol}/depth` | 获取订单簿深度 |

### 市场数据
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/api/v1/query/best-prices/{symbol}` | 获取最优价格 |
| GET | `/api/v1/query/trades/{symbol}` | 获取最近交易 |
| GET | `/api/v1/query/volume/{symbol}` | 获取交易量统计 |

## 📖 详细接口说明

### 1. 创建订单
**POST** `/api/v1/orders`

#### 请求示例
```bash
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440001",
    "symbol": "BTC/USD",
    "side": "Buy",
    "order_type": "Limit",
    "quantity": 100000000,
    "price": 50000000000,
    "time_in_force": "Gtc"
  }'
```

#### 响应示例
```json
{
  "success": true,
  "data": {
    "order_id": "550e8400-e29b-41d4-a716-446655440001",
    "status": "PENDING"
  },
  "error": null,
  "message": null
}
```

### 2. 查询订单
**GET** `/api/v1/orders/{order_id}`

```bash
curl http://localhost:8080/api/v1/orders/550e8400-e29b-41d4-a716-446655440001
```

#### 响应示例
```json
{
  "success": true,
  "data": {
    "order_id": "550e8400-e29b-41d4-a716-446655440001",
    "price": 50000000000,
    "quantity": 50000000,
    "side": "Buy",
    "symbol": "BTC/USD",
    "time_in_force": "Gtc"
  },
  "error": null,
  "message": null
}
```

### 3. 获取订单簿
**GET** `/api/v1/orderbook/{symbol}`

```bash
curl http://localhost:8080/api/v1/orderbook/BTC%2FUSD
```

#### 响应示例
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USD",
    "best_bid": 50000000000,
    "best_ask": null,
    "spread": null,
    "mid_price": null,
    "last_trade_price": 50000000000,
    "total_orders": 1,
    "bid_levels": 1,
    "ask_levels": 0,
    "total_bid_quantity": 50000000,
    "total_ask_quantity": 0,
    "timestamp": "2025-09-17T01:40:07.887559Z"
  },
  "error": null,
  "message": null
}
```

### 4. 获取最优价格
**GET** `/api/v1/query/best-prices/{symbol}`

```bash
curl http://localhost:8080/api/v1/query/best-prices/BTC%2FUSD
```

#### 响应示例
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USD",
    "best_bid": 50000000000,
    "best_ask": null,
    "spread": null,
    "mid_price": null,
    "timestamp": "2025-09-17T01:40:12.266385Z"
  },
  "error": null,
  "message": null
}
```

## 📊 数据格式

### 枚举值说明
- **side**: `Buy`, `Sell`
- **order_type**: `Limit`, `Market`
- **time_in_force**: `Gtc`, `Ioc`, `Fok`, `Day`

### 价格和数量精度
- 所有价格以最小货币单位表示（如：50000000000 = 500.00 USD）
- 所有数量以最小交易单位表示（如：100000000 = 1.00 BTC）

## 🔧 错误处理

所有接口都遵循统一的响应格式：

### 成功响应
```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "message": null
}
```

### 错误响应
```json
{
  "success": false,
  "data": null,
  "error": "错误描述信息",
  "message": null
}
```

## 📝 注意事项

1. **URL编码**: 交易对符号需要URL编码，如 `BTC/USD` → `BTC%2FUSD`
2. **用户ID**: 使用UUID格式
3. **订单ID**: 使用UUID格式
4. **状态码**: 所有请求都返回200，具体状态通过success字段判断

## 🔗 快速测试命令

```bash
# 创建订单
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id": "550e8400-e29b-41d4-a716-446655440001", "symbol": "BTC/USD", "side": "Buy", "order_type": "Limit", "quantity": 100000000, "price": 50000000000, "time_in_force": "Gtc"}'

# 查询订单
curl http://localhost:8080/api/v1/orders/550e8400-e29b-41d4-a716-446655440001

# 获取订单簿
curl http://localhost:8080/api/v1/orderbook/BTC%2FUSD

# 获取最优价格
curl http://localhost:8080/api/v1/query/best-prices/BTC%2FUSD
```

## 📚 更多接口

完整的接口文档请参考 [API_DOCUMENTATION.md](API_DOCUMENTATION.md)