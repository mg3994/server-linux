# 🚀 Multi-Vendor Delivery Server 🇮🇳

A high-performance, production-ready Rust backend for multi-vendor food delivery platforms, **specifically optimized for the Indian market**. Built with modern async patterns, comprehensive monitoring, enterprise-grade security, and full support for Indian payment methods, GST compliance, and regional preferences.

## ✨ Features

### 🔥 Core Capabilities
- **Modern Async Architecture**: Built with Tokio and Axum for maximum performance
- **🇮🇳 India-First Design**: Optimized for Indian market with UPI, GST, and regional features
- **💳 Indian Payment Methods**: UPI, Paytm, PhonePe, Google Pay, Net Banking, and more
- **📊 GST Integration**: Automatic GST calculation and compliance
- **🏛️ Regulatory Compliance**: FSSAI, RBI guidelines, and data localization
- **Firebase Authentication**: Secure JWT-based authentication with role-based access control
- **Real-time Communication**: WebSocket support for live order tracking and notifications
- **Push Notifications**: Modern FCM integration using OAuth 2.0 (no deprecated server keys)
- **Database Integration**: PostgreSQL with async SQLx for reliable data persistence
- **Comprehensive Monitoring**: Prometheus metrics and structured logging

### 🏗️ Architecture Highlights
- **Zero-cost Abstractions**: Leveraging Rust's performance guarantees
- **Type Safety**: Compile-time guarantees for data integrity
- **Async-first Design**: Non-blocking I/O for high concurrency
- **Modular Structure**: Clean separation of concerns for maintainability
- **Production Ready**: Comprehensive error handling and observability

### 📱 API Endpoints
- **Orders Management**: Create, track, and update order status with GST calculation
- **Delivery Management**: Complete delivery person lifecycle, order assignment, and real-time tracking
- **Payment Processing**: UPI, digital wallets, net banking, and traditional methods
- **User Management**: Customer, restaurant, and delivery person profiles
- **🇮🇳 India-Specific APIs**: Cities, states, cuisines, GST rates, UPI apps, banks, delivery zones
- **Real-time Updates**: WebSocket connections for live notifications and location tracking
- **Health Monitoring**: Detailed health checks and metrics endpoints
- **Admin Analytics**: Comprehensive delivery and business analytics

## 🚀 Quick Start

### Prerequisites
- **Rust**: 1.75 or later
- **PostgreSQL**: 13 or later
- **Firebase Project**: With authentication and FCM enabled

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd server
   ```

2. **Set up environment**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Set up database**
   ```bash
   createdb delivery_server
   sqlx migrate run
   ```

5. **Configure Firebase**
   - Download service account key from Firebase Console
   - Place as `firebase-service-account.json` in project root

6. **Run the server**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8443`

## 📚 Documentation

### Complete Guides
- **[📖 Complete Documentation](docs/COMPLETE_DOCUMENTATION.md)** - Comprehensive guide covering all aspects
- **[📚 API Reference](docs/API_REFERENCE.md)** - Detailed API documentation with examples
- **[🚀 Deployment Guide](docs/DEPLOYMENT_GUIDE.md)** - Production deployment instructions

### Quick References
- **[⚙️ Configuration](docs/COMPLETE_DOCUMENTATION.md#configuration)** - Environment variables and setup
- **[🔐 Security](docs/COMPLETE_DOCUMENTATION.md#security)** - Authentication and authorization
- **[📊 Monitoring](docs/COMPLETE_DOCUMENTATION.md#monitoring--metrics)** - Metrics and health checks
- **[🧪 Testing](docs/COMPLETE_DOCUMENTATION.md#testing)** - Testing strategies and examples

## 🏗️ Project Structure

```
server/
├── src/
│   ├── auth/                 # Authentication & authorization
│   ├── database/             # Database operations
│   ├── metrics/              # Prometheus metrics
│   ├── notifications/        # FCM push notifications
│   ├── orders/               # Order management
│   ├── payments/             # Payment processing
│   ├── websocket/            # Real-time communication
│   ├── config.rs             # Configuration management
│   ├── error.rs              # Error handling
│   ├── main.rs               # Application entry point
│   ├── routes.rs             # HTTP route definitions
│   └── server.rs             # Server setup and lifecycle
├── docs/                     # Comprehensive documentation
├── migrations/               # Database migrations
├── tests/                    # Integration tests
├── Cargo.toml               # Dependencies and metadata
└── README.md                # This file
```

## 🔧 Configuration

### Environment Variables

```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8443
RUST_LOG=info

# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/delivery_server_india

# Firebase Configuration (India Project)
FIREBASE_PROJECT_ID=your-india-delivery-project
FIREBASE_SERVICE_ACCOUNT_KEY=./firebase-service-account-india.json

# India-specific Configuration
DEFAULT_CURRENCY=INR
DEFAULT_TIMEZONE=Asia/Kolkata
DEFAULT_LANGUAGE=en-IN
COUNTRY=India

# GST Configuration
GST_ENABLED=true
GST_NUMBER=your-gst-number
FSSAI_LICENSE=your-fssai-license

# Payment Configuration
UPI_ENABLED=true
PAYTM_ENABLED=true
PHONEPE_ENABLED=true
GOOGLEPAY_ENABLED=true

# Delivery Configuration
MINIMUM_ORDER_AMOUNT=99.0
DELIVERY_FEE=29.0
FREE_DELIVERY_ABOVE=299.0

# Feature Flags
WEBSOCKET_ENABLED=true
NOTIFICATIONS_ENABLED=true
METRICS_ENABLED=true
INDIA_FEATURES_ENABLED=true
```

### Firebase Setup

1. Create a Firebase project at [Firebase Console](https://console.firebase.google.com/)
2. Enable Authentication and Cloud Messaging
3. Generate a service account key
4. Download and place the JSON file in your project root

## 🧪 Testing

### Run Tests
```bash
# All tests
cargo test

# Integration tests
cargo test --test integration_tests

# With coverage
cargo tarpaulin --out Html
```

### API Testing
```bash
# Health check
curl http://localhost:8443/health

# Detailed health with metrics
curl http://localhost:8443/health/detailed

# Prometheus metrics
curl http://localhost:8443/metrics
```

## 🚀 Deployment

### Docker
```bash
# Build and run with Docker Compose
docker-compose up -d

# Scale services
docker-compose up -d --scale server=3
```

### Kubernetes
```bash
# Deploy to Kubernetes
kubectl apply -f k8s/

# Check status
kubectl get pods -l app=delivery-server
```

### Cloud Platforms
- **AWS**: ECS, EKS, or Lambda deployment
- **Google Cloud**: Cloud Run or GKE deployment
- **Azure**: Container Instances or AKS deployment

See the [Deployment Guide](docs/DEPLOYMENT_GUIDE.md) for detailed instructions.

## 📊 Monitoring

### Health Endpoints
- `GET /health` - Basic health check
- `GET /health/detailed` - Detailed health with metrics
- `GET /metrics` - Prometheus metrics

### Key Metrics
- HTTP request rates and latencies
- Active orders and payment processing
- WebSocket connections
- Database performance
- System resource usage

### Logging
Structured JSON logging with configurable levels:
```bash
RUST_LOG=info,server=debug,sqlx=warn
```

## 🔐 Security

### Authentication
- **Firebase JWT**: Secure token-based authentication
- **Role-based Access**: Customer, restaurant, delivery person roles
- **Token Validation**: Automatic signature verification

### Security Features
- HTTPS/TLS encryption
- CORS configuration
- Input validation and sanitization
- SQL injection prevention
- Rate limiting support

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit changes**: `git commit -m 'Add amazing feature'`
4. **Push to branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Guidelines
- Follow Rust conventions and use `rustfmt`
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

### Getting Help
- **📖 Documentation**: Check the comprehensive guides first
- **🐛 Issues**: Create GitHub issues for bugs and feature requests
- **💬 Discussions**: Use GitHub Discussions for questions
- **📧 Email**: Contact the maintainers for enterprise support

### Reporting Issues
When reporting issues, please include:
- Environment details (OS, Rust version, dependencies)
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant log output (with sensitive data removed)

## 🎯 Roadmap

### Current Status: ✅ Production Ready
- ✅ Core API functionality
- ✅ Authentication and authorization
- ✅ Real-time features
- ✅ Monitoring and observability
- ✅ Comprehensive documentation

### Future Enhancements
- 🔄 Advanced caching strategies
- 📱 Mobile SDK integration
- 🤖 AI-powered recommendations
- 🌍 Multi-region deployment
- 📈 Advanced analytics

## 🏆 Acknowledgments

Built with modern Rust ecosystem:
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** - Database toolkit
- **[Serde](https://serde.rs/)** - Serialization framework
- **[Tracing](https://tracing.rs/)** - Structured logging
- **[Prometheus](https://prometheus.io/)** - Metrics collection

---

## 🎉 Ready to Deploy!

The Multi-Vendor Delivery Server is **production-ready** and provides a solid foundation for building scalable food delivery platforms. With comprehensive documentation, monitoring, and security features, it's designed to handle real-world demands while maintaining excellent developer experience.

**Start building your delivery platform today! 🚀**

---

*For detailed technical information, see the [Complete Documentation](docs/COMPLETE_DOCUMENTATION.md).*