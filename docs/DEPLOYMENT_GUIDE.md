# 🚀 Multi-Vendor Delivery Server - Deployment Guide

## 📋 Overview

This guide covers deploying the Multi-Vendor Delivery Server in various environments, from local development to production cloud deployments.

## 🛠️ Prerequisites

### System Requirements
- **CPU**: 2+ cores (4+ recommended for production)
- **RAM**: 4GB+ (8GB+ recommended for production)
- **Storage**: 20GB+ SSD
- **Network**: 100 Mbps+

### Software Dependencies
- **Rust**: 1.75 or later
- **PostgreSQL**: 13 or later
- **Docker**: 20.10+ (for containerized deployment)

## ⚙️ Environment Setup

### Environment Variables

Create a `.env` file:

```env
# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8443
RUST_LOG=info

# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/delivery_server
DATABASE_MAX_CONNECTIONS=100

# Firebase Configuration
FIREBASE_PROJECT_ID=your-firebase-project-id
FIREBASE_SERVICE_ACCOUNT_KEY=/path/to/service-account.json

# Security Configuration
CORS_ORIGINS=https://yourdomain.com
JWT_SECRET=your-jwt-secret-key

# Feature Flags
WEBSOCKET_ENABLED=true
NOTIFICATIONS_ENABLED=true
METRICS_ENABLED=true
```

## 🗄️ Database Setup

### PostgreSQL Installation

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### Docker
```bash
docker run --name postgres-delivery \
  -e POSTGRES_DB=delivery_server \
  -e POSTGRES_USER=delivery_user \
  -e POSTGRES_PASSWORD=secure_password \
  -p 5432:5432 \
  -v postgres_data:/var/lib/postgresql/data \
  -d postgres:15
```

### Database Configuration

```sql
-- Create database and user
CREATE DATABASE delivery_server;
CREATE USER delivery_user WITH ENCRYPTED PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE delivery_server TO delivery_user;

-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

### Run Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run --database-url postgresql://delivery_user:secure_password@localhost:5432/delivery_server
```

## 🔥 Firebase Configuration

### Setup Steps

1. **Create Firebase Project**
   - Go to [Firebase Console](https://console.firebase.google.com/)
   - Create new project
   - Enable Authentication and Cloud Messaging

2. **Generate Service Account Key**
   - Go to Project Settings > Service Accounts
   - Generate new private key
   - Download JSON file
   - Place in project root as `firebase-service-account.json`

## 💻 Local Development

### Quick Start

```bash
# Clone repository
git clone <repository-url>
cd server

# Setup environment
cp .env.example .env
# Edit .env with your configuration

# Install dependencies
cargo build

# Setup database
createdb delivery_server
sqlx migrate run

# Run development server
cargo run

# Or with auto-reload
cargo install cargo-watch
cargo watch -x run
```

## 🐳 Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8443

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8443/health || exit 1

CMD ["server"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  server:
    build: .
    ports:
      - "8443:8443"
    environment:
      - DATABASE_URL=postgresql://delivery_user:password@db:5432/delivery_server
      - FIREBASE_PROJECT_ID=your-project-id
      - FIREBASE_SERVICE_ACCOUNT_KEY=/app/firebase-service-account.json
      - RUST_LOG=info
    volumes:
      - ./firebase-service-account.json:/app/firebase-service-account.json:ro
    depends_on:
      db:
        condition: service_healthy
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=delivery_server
      - POSTGRES_USER=delivery_user
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U delivery_user -d delivery_server"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

volumes:
  postgres_data:
```

### Deploy with Docker

```bash
# Build and run
docker-compose up -d

# View logs
docker-compose logs -f server

# Scale services
docker-compose up -d --scale server=3

# Update deployment
docker-compose pull
docker-compose up -d
```

## ☸️ Kubernetes Deployment

### Deployment Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: delivery-server
  labels:
    app: delivery-server
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
              name: server-secrets
              key: DATABASE_URL
        - name: FIREBASE_PROJECT_ID
          valueFrom:
            secretKeyRef:
              name: server-secrets
              key: FIREBASE_PROJECT_ID
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
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

### Service Configuration

```yaml
apiVersion: v1
kind: Service
metadata:
  name: delivery-server-service
spec:
  selector:
    app: delivery-server
  ports:
  - name: http
    port: 80
    targetPort: 8443
  type: ClusterIP
```

### Deploy to Kubernetes

```bash
# Apply configurations
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml

# Check status
kubectl get pods
kubectl get services

# View logs
kubectl logs -f deployment/delivery-server

# Scale deployment
kubectl scale deployment delivery-server --replicas=5
```

## ☁️ Cloud Deployment

### AWS ECS

```json
{
  "family": "delivery-server",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "containerDefinitions": [
    {
      "name": "delivery-server",
      "image": "your-account.dkr.ecr.region.amazonaws.com/delivery-server:latest",
      "portMappings": [
        {
          "containerPort": 8443,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "RUST_LOG",
          "value": "info"
        }
      ],
      "secrets": [
        {
          "name": "DATABASE_URL",
          "valueFrom": "arn:aws:secretsmanager:region:account:secret:delivery-server/database-url"
        }
      ]
    }
  ]
}
```

### Google Cloud Run

```bash
# Build and push image
gcloud builds submit --tag gcr.io/your-project/delivery-server:latest

# Deploy to Cloud Run
gcloud run deploy delivery-server \
  --image gcr.io/your-project/delivery-server:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --memory 2Gi \
  --cpu 2 \
  --max-instances 10
```

## 📊 Monitoring & Logging

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'delivery-server'
    static_configs:
      - targets: ['delivery-server:8443']
    metrics_path: /metrics
```

### Logging Setup

```rust
// Configure structured logging
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}
```

## 🔒 Security Considerations

### Network Security

```bash
# Firewall configuration
ufw allow 22/tcp    # SSH
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw deny 8443/tcp   # Block direct access
ufw enable
```

### SSL/TLS Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;
    
    ssl_certificate /etc/ssl/certs/yourdomain.com.crt;
    ssl_certificate_key /etc/ssl/private/yourdomain.com.key;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    
    location / {
        proxy_pass http://delivery-server:8443;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## ⚡ Performance Optimization

### Application Tuning

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### Database Optimization

```sql
-- Add performance indexes
CREATE INDEX CONCURRENTLY idx_orders_customer_status ON orders(customer_id, status);
CREATE INDEX CONCURRENTLY idx_orders_restaurant_created ON orders(restaurant_id, created_at);
CREATE INDEX CONCURRENTLY idx_payments_order_status ON payments(order_id, status);
```

## 🔧 Troubleshooting

### Common Issues

#### High Memory Usage
```bash
# Check memory usage
docker stats delivery-server

# Monitor application metrics
curl http://localhost:8443/metrics | grep memory
```

#### Database Connection Issues
```bash
# Test database connectivity
pg_isready -h localhost -p 5432 -U delivery_user

# Check active connections
psql -U delivery_user -d delivery_server -c "SELECT count(*) FROM pg_stat_activity;"
```

#### Performance Issues
```bash
# Monitor system resources
htop
iotop

# Check application health
curl http://localhost:8443/health/detailed
```

### Recovery Procedures

#### Application Recovery
```bash
# Restart services
docker-compose restart server

# Scale up replicas
kubectl scale deployment delivery-server --replicas=5
```

#### Database Recovery
```bash
# Restore from backup
pg_restore -h localhost -U delivery_user -d delivery_server backup.dump
```

## 🎉 Conclusion

This deployment guide provides comprehensive instructions for deploying the Multi-Vendor Delivery Server in various environments. The server is designed to be production-ready with proper monitoring, security, and performance optimizations.

### Key Benefits

- **🚀 Scalable**: Horizontal scaling support
- **🔒 Secure**: TLS encryption and authentication
- **📊 Observable**: Comprehensive metrics and logging
- **⚡ Performant**: Optimized for high throughput
- **🛡️ Reliable**: Health checks and auto-recovery

For additional support, refer to the [Complete Documentation](./COMPLETE_DOCUMENTATION.md) and [API Reference](./API_REFERENCE.md).

**Happy deploying! 🚀**