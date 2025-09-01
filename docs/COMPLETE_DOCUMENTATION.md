# 🚀 Multi-Vendor Delivery Server - Complete Documentation

## 📋 Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Features](#features)
4. [Installation & Setup](#installation--setup)
5. [API Reference](#api-reference)
6. [WebSocket API](#websocket-api)
7. [Database Schema](#database-schema)
8. [Configuration](#configuration)
9. [Monitoring & Metrics](#monitoring--metrics)
10. [Security](#security)
11. [Deployment](#deployment)
12. [Development Guide](#development-guide)
13. [Testing](#testing)
14. [Troubleshooting](#troubleshooting)

## 🎯 Overview

The Multi-Vendor Delivery Server is a high-performance, production-ready Rust application built with modern async patterns. It provides a complete backend solution for food delivery platforms supporting multiple restaurants, customers, and delivery personnel.

### Key Highlights

- **🔥 Modern Rust**: Built with Tokio async runtime and Axum web framework
- **🚀 High Performance**: Zero-cost abstractions and efficient resource usage
- **🔐 Secure**: Firebase JWT authentication with role-based access control
- **📱 Real-time**: WebSocket support for live order tracking and notifications
- **📊 Observable**: Comprehensive metrics and health monitoring
- **🌐 Scalable**: Designed for horizontal scaling and microservices architecture
- **📡 Modern FCM**: OAuth 2.0 based push notifications (no deprecated server keys)

## 🏗️ Architecture

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Mobile Apps   │    │   Web Frontend  │    │  Admin Panel    │
│  (iOS/Android)  │    │   (React/Vue)   │    │   (Dashboard)   │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │     Load Balancer        │
                    │    (Nginx/HAProxy)       │
                    └─────────────┬─────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                      │
    ┌─────▼─────┐          ┌─────▼─────┐          ┌─────▼─────┐
    │  Server   │          │  Server   │          │  Server   │
    │ Instance  │          │ Instance  │          │ Instance  │
    │    #1     │          │    #2     │          │    #3     │
    └─────┬─────┘          └─────┬─────┘          └─────┬─────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │      PostgreSQL          │
                    │     (Primary DB)         │
                    └──────────────────────────┘
```

### Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Multi-Vendor Delivery Server             │
├─────────────────────────────────────────────────────────────┤
│  HTTP/WebSocket Layer (Axum)                               │
├─────────────────────────────────────────────────────────────┤
│  Authentication Middleware (Firebase JWT)                   │
├─────────────────────────────────────────────────────────────┤
│  Business Logic Layer                                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │   Orders    │ │  Payments   │ │ Notifications│           │
│  │   Service   │ │   Service   │ │   Service    │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
├─────────────────────────────────────────────────────────────┤
│  Data Access Layer                                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │  Database   │ │  WebSocket  │ │   Metrics   │           │
│  │   Layer     │ │   Manager   │ │ Collector   │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
├─────────────────────────────────────────────────────────────┤
│  External Services                                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │  Firebase   │ │     FCM     │ │ PostgreSQL  │           │
│  │    Auth     │ │ Notifications│ │  Database   │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
└─────────────────────────────────────────────────────────────┘
```

## ✨ Features

### Core Features

#### 🔐 Authentication & Authorization
- **Firebase JWT Verification**: Secure token-based authentication
- **Role-based Access Control**: Customer, Restaurant, Delivery Person roles
- **Email & Phone Verification**: Dual verification requirements
- **Token Refresh**: Automatic token renewal handling

#### 📦 Order Management
- **Order Creation**: Complete order workflow with item management
- **Status Tracking**: Real-time order status updates
- **Order History**: Customer order history and details
- **Multi-vendor Support**: Handle orders from multiple restaurants

#### 💳 Payment Processing
- **Payment Creation**: Secure payment processing workflow
- **Payment Status**: Real-time payment status tracking
- **Multiple Payment Methods**: Support for various payment options
- **Transaction History**: Complete payment audit trail

#### 📱 Push Notifications
- **Modern FCM Integration**: OAuth 2.0 based (no deprecated server keys)
- **Role-based Notifications**: Customized messages for different user types
- **Batch Processing**: Efficient concurrent notification sending
- **Delivery Confirmations**: Automatic retry and error handling

#### 🌐 Real-time Communication
- **WebSocket Support**: Live order tracking and updates
- **Location Updates**: Real-time delivery person location sharing
- **Instant Notifications**: Push notifications through WebSocket
- **Connection Management**: Automatic reconnection and cleanup

#### 📊 Monitoring & Observability
- **Prometheus Metrics**: Comprehensive application metrics
- **Health Checks**: Basic and detailed health endpoints
- **Structured Logging**: Tracing with configurable log levels
- **Performance Monitoring**: Request duration and throughput metrics

### Advanced Features

#### 🗄️ Database Integration
- **PostgreSQL Support**: Async database operations with SQLx
- **Connection Pooling**: Efficient database connection management
- **Migration Support**: Database schema versioning
- **Transaction Support**: ACID compliance for critical operations

#### 🔧 Configuration Management
- **Environment Variables**: Flexible configuration options
- **Hot Reloading**: Configuration updates without restart
- **Validation**: Configuration validation at startup
- **Defaults**: Sensible default values for all settings

#### 🚀 Performance Optimization
- **Async Runtime**: Tokio-based high-performance async execution
- **Connection Pooling**: Efficient resource utilization
- **Caching**: In-memory caching for frequently accessed data
- **Batch Operations**: Optimized bulk operations

## 🛠️ Installation & Setup

### Prerequisites

- **Rust**: 1.75 or later
- **PostgreSQL**: 13 or later
- **Firebase Project**: With authentication enabled
- **FCM Service Account**: For push notifications

### Quick Start

1. **Clone the Repository**
   ```bash
   git clone <repository-url>
   cd server
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Set Up Environment**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Database Setup**
   ```bash
   # Create database
   createdb delivery_server
   
   # Run migrations (when implemented)
   sqlx migrate run
   ```

5. **Firebase Setup**
   - Create a Firebase project
   - Enable Authentication
   - Download service account key
   - Place in project root as `firebase-service-account.json`

6. **Run the Server**
   ```bash
   cargo run
   ```

### Environment Configuration

Create a `.env` file with the following variables:

```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8443
RUST_LOG=info

# Database Configuration
DATABASE_URL=postgresql://username:password@localhost/delivery_server

# Firebase Configuration
FIREBASE_PROJECT_ID=your-project-id
FIREBASE_SERVICE_ACCOUNT_KEY=./firebase-service-account.json

# Optional Configuration
MAX_CONNECTIONS=100
ENABLE_METRICS=true
CORS_ORIGINS=*
```

## 📚 API Reference

### Base URL
```
https://api.yourdeliveryapp.com
```

### Authentication
All protected endpoints require a Bearer token in the Authorization header:
```
Authorization: Bearer <firebase-jwt-token>
```

### Endpoints

#### Health & Monitoring

##### GET /health
Basic health check endpoint.

**Response:**
```
OK
```

##### GET /health/detailed
Detailed health information with metrics.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime_seconds": 3600,
  "version": "0.1.0",
  "build_info": "server@0.1.0",
  "metrics": {
    "http_requests_total": 1250,
    "active_orders": 45,
    "websocket_connections": 23,
    "notifications_sent_total": 890,
    "payments_processed_total": 156
  }
}
```

##### GET /metrics
Prometheus metrics endpoint.

**Response:**
```
# HELP http_requests_total Total number of HTTP requests
# TYPE http_requests_total counter
http_requests_total{service="delivery_server"} 1250
...
```

#### Orders

##### POST /orders
Create a new order.

**Request Body:**
```json
{
  "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
  "items": [
    {
      "menu_item_id": "456e7890-e89b-12d3-a456-426614174001",
      "quantity": 2,
      "customizations": ["No onions", "Extra cheese"]
    }
  ],
  "delivery_address": {
    "street": "123 Main St",
    "city": "Anytown",
    "state": "ST",
    "postal_code": "12345",
    "country": "US",
    "latitude": 40.7128,
    "longitude": -74.0060
  }
}
```

**Response:**
```json
{
  "order": {
    "id": "789e0123-e89b-12d3-a456-426614174002",
    "customer_id": "user123",
    "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
    "status": "placed",
    "total_amount": 25.99,
    "items": [...],
    "delivery_address": {...},
    "created_at": "2024-01-15T10:30:00Z",
    "estimated_delivery_time": "2024-01-15T11:00:00Z"
  },
  "message": "Order created successfully"
}
```

##### GET /orders/:id
Get order details by ID.

**Response:**
```json
{
  "id": "789e0123-e89b-12d3-a456-426614174002",
  "customer_id": "user123",
  "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
  "delivery_person_id": "delivery456",
  "status": "preparing",
  "total_amount": 25.99,
  "items": [
    {
      "id": "item1",
      "menu_item_id": "456e7890-e89b-12d3-a456-426614174001",
      "name": "Margherita Pizza",
      "quantity": 1,
      "unit_price": 15.99,
      "total_price": 15.99,
      "customizations": ["Extra cheese"]
    }
  ],
  "delivery_address": {...},
  "restaurant_address": {...},
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:35:00Z",
  "estimated_delivery_time": "2024-01-15T11:00:00Z"
}
```

##### PUT /orders/:id/status
Update order status.

**Request Body:**
```json
{
  "status": "ready"
}
```

**Response:**
```json
{
  "order": {
    "id": "789e0123-e89b-12d3-a456-426614174002",
    "status": "ready",
    "updated_at": "2024-01-15T10:45:00Z",
    ...
  },
  "message": "Order status updated successfully"
}
```

##### GET /customers/:id/orders
Get all orders for a customer.

**Response:**
```json
[
  {
    "id": "order1",
    "status": "delivered",
    "total_amount": 25.99,
    "created_at": "2024-01-15T09:00:00Z",
    ...
  },
  {
    "id": "order2",
    "status": "preparing",
    "total_amount": 18.50,
    "created_at": "2024-01-15T10:30:00Z",
    ...
  }
]
```

##### PUT /orders/:id/location
Update delivery location (for delivery personnel).

**Request Body:**
```json
{
  "customer_id": "user123",
  "latitude": 40.7128,
  "longitude": -74.0060
}
```

**Response:**
```json
{
  "success": true,
  "message": "Location update sent"
}
```

#### Payments

##### POST /payments
Process a payment.

**Request Body:**
```json
{
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "amount": 25.99,
  "currency": "USD",
  "payment_method": "credit_card",
  "payment_details": {
    "card_token": "tok_1234567890",
    "billing_address": {...}
  }
}
```

**Response:**
```json
{
  "payment": {
    "id": "pay_123456789",
    "order_id": "789e0123-e89b-12d3-a456-426614174002",
    "amount": 25.99,
    "currency": "USD",
    "status": "completed",
    "payment_method": "credit_card",
    "created_at": "2024-01-15T10:30:00Z"
  },
  "message": "Payment processed successfully"
}
```

##### GET /payments/:id
Get payment details by ID.

**Response:**
```json
{
  "id": "pay_123456789",
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "amount": 25.99,
  "currency": "USD",
  "status": "completed",
  "payment_method": "credit_card",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:05Z"
}
```

## 🌐 WebSocket API

### Connection
Connect to WebSocket endpoint with authentication:
```
wss://api.yourdeliveryapp.com/ws
Authorization: Bearer <firebase-jwt-token>
```

### Message Types

#### Order Status Updates
Sent when order status changes:
```json
{
  "type": "order_status_update",
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "status": "preparing",
  "timestamp": "2024-01-15T10:35:00Z"
}
```

#### Delivery Location Updates
Sent when delivery person location changes:
```json
{
  "type": "delivery_location_update",
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "latitude": 40.7128,
  "longitude": -74.0060,
  "timestamp": "2024-01-15T10:40:00Z"
}
```

#### Notifications
General notifications:
```json
{
  "type": "notification",
  "title": "Order Update",
  "message": "Your order is ready for pickup!",
  "timestamp": "2024-01-15T10:45:00Z"
}
```

#### Ping/Pong
Keep-alive messages:
```json
{
  "type": "ping"
}
```

```json
{
  "type": "pong"
}
```

## 🗄️ Database Schema

### Tables

#### users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    firebase_uid VARCHAR(128) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    role VARCHAR(20) NOT NULL CHECK (role IN ('customer', 'restaurant', 'delivery_person')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### orders
```sql
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES users(id),
    restaurant_id UUID NOT NULL REFERENCES users(id),
    delivery_person_id UUID REFERENCES users(id),
    status VARCHAR(20) NOT NULL DEFAULT 'placed',
    total_amount DECIMAL(10,2) NOT NULL,
    items JSONB NOT NULL,
    delivery_address JSONB NOT NULL,
    restaurant_address JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    estimated_delivery_time TIMESTAMP WITH TIME ZONE
);
```

#### payments
```sql
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id),
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    payment_method VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### notifications
```sql
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50) NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Indexes

```sql
-- Performance indexes
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_restaurant_id ON orders(restaurant_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_read ON notifications(read);
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SERVER_HOST` | Server bind address | `127.0.0.1` | No |
| `SERVER_PORT` | Server port | `8443` | No |
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `FIREBASE_PROJECT_ID` | Firebase project ID | - | Yes |
| `FIREBASE_SERVICE_ACCOUNT_KEY` | Path to service account JSON | - | Yes |
| `RUST_LOG` | Log level | `info` | No |
| `MAX_CONNECTIONS` | Max database connections | `100` | No |
| `ENABLE_METRICS` | Enable Prometheus metrics | `true` | No |
| `CORS_ORIGINS` | CORS allowed origins | `*` | No |

### Firebase Configuration

1. **Create Firebase Project**
   - Go to [Firebase Console](https://console.firebase.google.com/)
   - Create a new project
   - Enable Authentication

2. **Generate Service Account Key**
   - Go to Project Settings > Service Accounts
   - Generate new private key
   - Download JSON file
   - Place in project root

3. **Configure FCM**
   - Enable Cloud Messaging
   - Note the project ID for configuration

### Database Configuration

1. **Install PostgreSQL**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install postgresql postgresql-contrib
   
   # macOS
   brew install postgresql
   
   # Windows
   # Download from https://www.postgresql.org/download/windows/
   ```

2. **Create Database**
   ```bash
   createdb delivery_server
   ```

3. **Set Connection String**
   ```env
   DATABASE_URL=postgresql://username:password@localhost/delivery_server
   ```

## 📊 Monitoring & Metrics

### Prometheus Metrics

The server exposes comprehensive metrics at `/metrics`:

#### HTTP Metrics
- `http_requests_total`: Total HTTP requests
- `http_request_duration_seconds`: Request duration histogram
- `http_requests_in_flight`: Current active requests

#### Business Metrics
- `orders_created_total`: Total orders created
- `orders_completed_total`: Total orders completed
- `orders_cancelled_total`: Total orders cancelled
- `active_orders`: Current active orders
- `payments_processed_total`: Total payments processed
- `payments_failed_total`: Total failed payments
- `payment_amount_total`: Total payment amount

#### System Metrics
- `websocket_connections`: Active WebSocket connections
- `notifications_sent_total`: Total notifications sent
- `notifications_failed_total`: Total failed notifications

### Health Checks

#### Basic Health Check
```bash
curl http://localhost:8443/health
```

#### Detailed Health Check
```bash
curl http://localhost:8443/health/detailed
```

### Logging

The server uses structured logging with tracing:

```rust
// Log levels: error, warn, info, debug, trace
RUST_LOG=info

// Module-specific logging
RUST_LOG=server=debug,sqlx=warn

// JSON logging for production
RUST_LOG=info,server::orders=debug
```

## 🔐 Security

### Authentication Flow

1. **Client Authentication**
   - User signs in with Firebase Auth
   - Client receives JWT token
   - Token included in API requests

2. **Server Verification**
   - Server validates JWT signature
   - Checks token expiration
   - Extracts user information
   - Injects user context

3. **Authorization**
   - Role-based access control
   - Resource ownership validation
   - Permission checking

### Security Best Practices

#### JWT Token Security
- Tokens are verified using Firebase public keys
- Automatic key rotation support
- Token expiration validation
- Secure token storage on client

#### API Security
- HTTPS only in production
- CORS configuration
- Rate limiting (recommended)
- Input validation and sanitization

#### Database Security
- Connection pooling with limits
- Prepared statements (SQL injection prevention)
- Row-level security (when implemented)
- Audit logging

### Security Headers

```rust
// Recommended security headers
"X-Content-Type-Options": "nosniff"
"X-Frame-Options": "DENY"
"X-XSS-Protection": "1; mode=block"
"Strict-Transport-Security": "max-age=31536000"
```

## 🚀 Deployment

### Docker Deployment

#### Dockerfile
```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /app/firebase-service-account.json /app/

EXPOSE 8443

CMD ["server"]
```

#### Docker Compose
```yaml
version: '3.8'

services:
  server:
    build: .
    ports:
      - "8443:8443"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/delivery_server
      - FIREBASE_PROJECT_ID=your-project-id
      - FIREBASE_SERVICE_ACCOUNT_KEY=/app/firebase-service-account.json
      - RUST_LOG=info
    depends_on:
      - db
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=delivery_server
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  prometheus:
    image: prom/prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    restart: unless-stopped

volumes:
  postgres_data:
```

### Kubernetes Deployment

#### Deployment YAML
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: delivery-server
spec:
  replicas: 3
  selector:
    matchLabels:
      app: delivery-server
  template:
    metadata:
      labels:
        app: delivery-server
    spec:
      containers:
      - name: server
        image: your-registry/delivery-server:latest
        ports:
        - containerPort: 8443
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        - name: FIREBASE_PROJECT_ID
          value: "your-project-id"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8443
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8443
          initialDelaySeconds: 5
          periodSeconds: 5
```

### Production Considerations

#### Performance Tuning
- **Database Connection Pool**: Tune `MAX_CONNECTIONS` based on load
- **Worker Threads**: Configure Tokio runtime threads
- **Memory Limits**: Set appropriate container memory limits
- **CPU Limits**: Configure CPU limits for consistent performance

#### Scaling
- **Horizontal Scaling**: Multiple server instances behind load balancer
- **Database Scaling**: Read replicas for read-heavy workloads
- **Caching**: Redis for session and frequently accessed data
- **CDN**: Static asset delivery

#### Monitoring
- **Application Metrics**: Prometheus + Grafana
- **Log Aggregation**: ELK stack or similar
- **Error Tracking**: Sentry or similar service
- **Uptime Monitoring**: External monitoring service

## 👨‍💻 Development Guide

### Project Structure

```
server/
├── src/
│   ├── auth/                 # Authentication module
│   │   ├── firebase.rs       # Firebase JWT verification
│   │   ├── middleware.rs     # Auth middleware
│   │   └── models.rs         # User models
│   ├── database/             # Database layer
│   │   └── mod.rs           # Database operations
│   ├── metrics/              # Metrics collection
│   │   └── mod.rs           # Prometheus metrics
│   ├── notifications/        # Push notifications
│   │   ├── fcm.rs           # FCM service
│   │   └── models.rs        # Notification models
│   ├── orders/               # Order management
│   │   ├── handlers.rs      # HTTP handlers
│   │   ├── models.rs        # Order models
│   │   └── mod.rs           # Module exports
│   ├── payments/             # Payment processing
│   │   ├── handlers.rs      # HTTP handlers
│   │   └── models.rs        # Payment models
│   ├── websocket/            # WebSocket support
│   │   └── mod.rs           # WebSocket handlers
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error handling
│   ├── lib.rs               # Library root
│   ├── main.rs              # Application entry
│   ├── middleware.rs        # HTTP middleware
│   ├── routes.rs            # Route definitions
│   └── server.rs            # Server setup
├── docs/                    # Documentation
├── migrations/              # Database migrations
├── tests/                   # Test files
├── Cargo.toml              # Dependencies
├── .env.example            # Environment template
└── README.md               # Project README
```

### Development Workflow

1. **Setup Development Environment**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development tools
   cargo install cargo-watch sqlx-cli
   
   # Clone and setup
   git clone <repo>
   cd server
   cp .env.example .env
   ```

2. **Database Development**
   ```bash
   # Create migration
   sqlx migrate add create_users_table
   
   # Run migrations
   sqlx migrate run
   
   # Generate query metadata
   cargo sqlx prepare
   ```

3. **Development Server**
   ```bash
   # Watch mode with auto-reload
   cargo watch -x run
   
   # Run with debug logging
   RUST_LOG=debug cargo run
   ```

4. **Testing**
   ```bash
   # Run all tests
   cargo test
   
   # Run specific test
   cargo test test_create_order
   
   # Run with output
   cargo test -- --nocapture
   ```

### Code Style

#### Rust Conventions
- Use `rustfmt` for formatting
- Follow Rust naming conventions
- Use `clippy` for linting
- Document public APIs

#### Error Handling
```rust
// Use Result types for fallible operations
pub async fn create_order(payload: CreateOrderRequest) -> Result<Order> {
    // Implementation
}

// Use custom error types
#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Order not found")]
    NotFound,
    #[error("Invalid order status: {0}")]
    InvalidStatus(String),
}
```

#### Async Patterns
```rust
// Use async/await for I/O operations
pub async fn get_order(id: Uuid) -> Result<Order> {
    let order = database.get_order(id).await?;
    Ok(order)
}

// Use Arc<Mutex<T>> for shared mutable state
pub type SharedFCMService = Arc<Mutex<FCMService>>;
```

### Adding New Features

1. **Create Module Structure**
   ```bash
   mkdir src/new_feature
   touch src/new_feature/mod.rs
   touch src/new_feature/handlers.rs
   touch src/new_feature/models.rs
   ```

2. **Define Models**
   ```rust
   // src/new_feature/models.rs
   use serde::{Deserialize, Serialize};
   use uuid::Uuid;
   
   #[derive(Debug, Serialize, Deserialize)]
   pub struct NewFeature {
       pub id: Uuid,
       pub name: String,
       // ... other fields
   }
   ```

3. **Implement Handlers**
   ```rust
   // src/new_feature/handlers.rs
   use axum::{Json, extract::Path};
   use crate::error::Result;
   
   pub async fn create_feature(
       Json(payload): Json<CreateFeatureRequest>,
   ) -> Result<Json<NewFeature>> {
       // Implementation
   }
   ```

4. **Add Routes**
   ```rust
   // src/routes.rs
   .route("/features", post(create_feature))
   .route("/features/:id", get(get_feature))
   ```

5. **Update Module Exports**
   ```rust
   // src/lib.rs
   pub mod new_feature;
   ```

## 🧪 Testing

### Test Structure

```
tests/
├── integration/
│   ├── auth_tests.rs
│   ├── order_tests.rs
│   └── payment_tests.rs
├── unit/
│   ├── models_tests.rs
│   └── utils_tests.rs
└── common/
    └── mod.rs
```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_creation() {
        let order = Order::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            vec![],
            25.99,
        );
        
        assert_eq!(order.status, OrderStatus::Placed);
        assert_eq!(order.total_amount, 25.99);
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        let result = some_async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
// tests/integration/order_tests.rs
use server::*;
use axum_test::TestServer;

#[tokio::test]
async fn test_create_order_endpoint() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();
    
    let response = server
        .post("/orders")
        .json(&serde_json::json!({
            "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
            "items": []
        }))
        .await;
    
    assert_eq!(response.status_code(), 201);
}
```

### Test Database

```rust
// tests/common/mod.rs
use sqlx::PgPool;

pub async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/delivery_server_test".to_string());
    
    let pool = PgPool::connect(&database_url).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    
    pool
}
```

### Running Tests

```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration

# Unit tests only
cargo test --lib

# Specific test
cargo test test_create_order

# With output
cargo test -- --nocapture

# Test coverage
cargo tarpaulin --out Html
```

## 🔧 Troubleshooting

### Common Issues

#### Compilation Errors

**Issue**: Missing dependencies
```
error[E0432]: unresolved import `sqlx`
```

**Solution**: Add dependency to Cargo.toml
```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
```

**Issue**: Async runtime errors
```
error: `async fn` is not permitted in the current context
```

**Solution**: Use `#[tokio::main]` or `tokio::spawn`
```rust
#[tokio::main]
async fn main() {
    // async code here
}
```

#### Runtime Errors

**Issue**: Database connection failed
```
Error: Failed to connect to database
```

**Solution**: Check database configuration
```bash
# Verify database is running
pg_isready -h localhost -p 5432

# Check connection string
echo $DATABASE_URL

# Test connection
psql $DATABASE_URL
```

**Issue**: Firebase authentication failed
```
Error: Invalid Firebase token
```

**Solution**: Verify Firebase configuration
```bash
# Check service account file exists
ls -la firebase-service-account.json

# Verify project ID
grep project_id firebase-service-account.json

# Check environment variables
echo $FIREBASE_PROJECT_ID
```

#### Performance Issues

**Issue**: High memory usage
```
Server consuming excessive memory
```

**Solution**: Tune configuration
```env
# Reduce database connections
MAX_CONNECTIONS=50

# Limit request body size
MAX_REQUEST_SIZE=1048576
```

**Issue**: Slow database queries
```
Database queries taking too long
```

**Solution**: Add indexes and optimize queries
```sql
-- Add indexes for frequently queried columns
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);

-- Analyze query performance
EXPLAIN ANALYZE SELECT * FROM orders WHERE customer_id = $1;
```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
# Debug all modules
RUST_LOG=debug cargo run

# Debug specific modules
RUST_LOG=server::orders=debug,sqlx=info cargo run

# Trace level (very verbose)
RUST_LOG=trace cargo run
```

### Health Checks

Use health endpoints to diagnose issues:

```bash
# Basic health check
curl http://localhost:8443/health

# Detailed health with metrics
curl http://localhost:8443/health/detailed | jq

# Prometheus metrics
curl http://localhost:8443/metrics
```

### Log Analysis

Common log patterns to look for:

```bash
# Error patterns
grep "ERROR" logs/server.log

# Database connection issues
grep "database" logs/server.log | grep -i error

# Authentication failures
grep "auth" logs/server.log | grep -i "failed\|error"

# Performance issues
grep "duration" logs/server.log | sort -k3 -nr
```

## 📞 Support

### Getting Help

1. **Documentation**: Check this comprehensive guide first
2. **Issues**: Create GitHub issues for bugs and feature requests
3. **Discussions**: Use GitHub Discussions for questions
4. **Community**: Join our Discord/Slack community

### Contributing

1. **Fork the Repository**
2. **Create Feature Branch**: `git checkout -b feature/amazing-feature`
3. **Commit Changes**: `git commit -m 'Add amazing feature'`
4. **Push to Branch**: `git push origin feature/amazing-feature`
5. **Open Pull Request**

### Reporting Issues

When reporting issues, please include:

- **Environment**: OS, Rust version, dependencies
- **Configuration**: Relevant environment variables (redacted)
- **Steps to Reproduce**: Clear reproduction steps
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Logs**: Relevant log output (with sensitive data removed)

---

## 🎉 Conclusion

The Multi-Vendor Delivery Server provides a robust, scalable foundation for building modern food delivery platforms. With its comprehensive feature set, excellent performance characteristics, and production-ready architecture, it's designed to handle real-world demands while maintaining code quality and developer productivity.

**Key Strengths:**
- ✅ **Production-Ready**: Comprehensive error handling, logging, and monitoring
- ✅ **Modern Architecture**: Async Rust with clean, maintainable code
- ✅ **Scalable Design**: Built for horizontal scaling and microservices
- ✅ **Security First**: Firebase JWT authentication with role-based access
- ✅ **Real-time Features**: WebSocket support for live updates
- ✅ **Observable**: Prometheus metrics and structured logging
- ✅ **Well-Documented**: Comprehensive documentation and examples

The server is ready for immediate deployment and can serve as the backbone for a complete multi-vendor delivery platform. Its modular design makes it easy to extend and customize for specific business requirements.

**Happy coding! 🚀**