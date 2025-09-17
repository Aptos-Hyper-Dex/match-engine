# OrderBook API 文档

## 基础信息
- **Base URL**: `http://localhost:8080`
- **API版本**: `/api/v1`
- **内容类型**: `application/json`

## 订单相关接口

### 1. 创建订单
**POST** `/api/v1/orders`

创建新的交易订单。

#### 请求参数
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440001",
  "symbol": "BTC/USD",
  "side": "Buy",        // 可选值: Buy, Sell
  "order_type": "Limit", // 可选值: Limit, Market
  "quantity": 100000000,
  "price": 50000000000,  // Limit订单必填
  "time_in_force": "Gtc" // 可选值: Gtc, Ioc, Fok, Day
}
```

#### 响应示例
**成功 (200 OK)**
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

**错误响应**
```json
{
  "success": false,
  "data": null,
  "error": "Json deserialize error: unknown variant `BUY`, expected `Buy` or `Sell`",
  "message": null
}
```

---

### 2. 查询订单详情
**GET** `/api/v1/orders/{order_id}`

获取指定订单的详细信息。

#### 路径参数
- `order_id` (string, required): 订单ID

#### 响应示例
**成功 (200 OK)**
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

**订单不存在 (200 OK)**
```json
{
  "success": false,
  "data": null,
  "error": "order not found",
  "message": null
}
```

---

### 3. 更新订单
**PUT** `/api/v1/orders/{order_id}`

更新指定订单的信息。

#### 路径参数
- `order_id` (string, required): 订单ID

#### 请求参数
```json
{
  "quantity": 150000000,
  "price": 3100000000
}
```

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "updated": true
  },
  "error": null,
  "message": null
}
```

---

### 4. 取消订单
**DELETE** `/api/v1/orders/{order_id}`

取消指定的订单。

#### 路径参数
- `order_id` (string, required): 订单ID

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "cancelled": true
  },
  "error": null,
  "message": null
}
```

---

### 5. 查询用户订单列表
**GET** `/api/v1/orders/user/{user_id}`

获取指定用户的所有订单列表。

#### 路径参数
- `user_id` (string, required): 用户ID

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "orders": [],
    "total": 0
  },
  "error": null,
  "message": null
}
```

---

## 订单簿相关接口

### 6. 获取所有订单簿
**GET** `/api/v1/orderbook`

获取所有交易对的订单簿概览。

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": [
    {
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
      "timestamp": "2025-09-17T01:50:55.992392Z"
    }
  ],
  "error": null,
  "message": null
}
```

---

### 7. 获取指定订单簿
**GET** `/api/v1/orderbook/{symbol}`

获取指定交易对的订单簿信息。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码（如 `BTC%2FUSD`）

#### 响应示例
**成功 (200 OK)**
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

---

### 8. 获取订单簿快照
**GET** `/api/v1/orderbook/{symbol}/snapshot`

获取指定交易对的订单簿快照，包含详细的买卖盘口信息。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USD",
    "timestamp": "2025-09-17T01:51:00.858Z",
    "bids": [
      {
        "price": 50000000000,
        "visible_quantity": 50000000,
        "hidden_quantity": 0,
        "order_count": 1
      }
    ],
    "asks": []
  },
  "error": null,
  "message": null
}
```

---

### 9. 获取订单簿深度
**GET** `/api/v1/orderbook/{symbol}/depth`

获取指定交易对的订单簿深度信息。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USD",
    "bids": [
      {
        "price": 50000000000,
        "visible_quantity": 50000000,
        "hidden_quantity": 0,
        "order_count": 1
      }
    ],
    "asks": [],
    "timestamp": "2025-09-17T01:51:06.438Z"
  },
  "error": null,
  "message": null
}
```

---

## 市场数据查询接口

### 10. 获取最优价格
**GET** `/api/v1/query/best-prices/{symbol}`

获取指定交易对的最优买卖价格。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码

#### 响应示例
**成功 (200 OK)**
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

---

### 11. 获取最近交易
**GET** `/api/v1/query/trades/{symbol}`

获取指定交易对的最近交易记录。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "trades": [],
    "total": 0,
    "page": 1,
    "page_size": 50
  },
  "error": null,
  "message": null
}
```

---

### 12. 获取交易量统计
**GET** `/api/v1/query/volume/{symbol}`

获取指定交易对的交易量统计信息。

#### 路径参数
- `symbol` (string, required): 交易对符号，需要URL编码

#### 响应示例
**成功 (200 OK)**
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USD",
    "total_volume": 0,
    "total_trades": 0,
    "avg_price": 0.0,
    "high_price": 0,
    "low_price": 0,
    "timestamp": "2025-09-17T01:51:11.870965Z"
  },
  "error": null,
  "message": null
}
```

---

## 错误处理

所有接口都遵循统一的错误响应格式：

```json
{
  "success": false,
  "data": null,
  "error": "错误描述信息",
  "message": null
}
```

## 数据类型说明

### 价格精度
- 所有价格都以最小货币单位表示（如聪/聪）
- 例如：50000000000 表示 500.00 USD

### 数量精度
- 所有数量都以最小交易单位表示
- 例如：100000000 表示 1.00 BTC

### 枚举值
- **side**: `Buy`, `Sell`
- **order_type**: `Limit`, `Market`
- **time_in_force**: `Gtc`, `Ioc`, `Fok`, `Day`

## 状态码说明
- **200 OK**: 请求成功
- **404 Not Found**: 接口不存在
- **其他状态码**: 遵循标准HTTP状态码规范