# 📚 API Reference

## Overview

The Multi-Vendor Delivery Server provides a comprehensive REST API for managing orders, users, restaurants, delivery personnel, and payments. All endpoints use JSON for request and response bodies.

## Base URL

```
Production: https://api.delivery-platform.com
Development: http://localhost:8443
```

## Authentication

All API endpoints (except `/health`) require Firebase JWT authentication.

**Header Format:**
```http
Authorization: Bearer <firebase-jwt-token>
```

**User Requirements:**
- Email must be verified
- Phone number must be verified
- User must have appropriate role for the endpoint

## Response Format

### Success Response
```json
{
  "data": { ... },
  "timestamp": "2024-01-15T10:30:00Z",
  "request_id": "uuid"
}
```

### Error Response
```json
{
  "error": "error_code",
  "message": "Human readable error message",
  "details": { ... },
  "timestamp": "2024-01-15T10:30:00Z",
  "request_id": "uuid"
}
```

## Status Codes

| Code | Description |
|------|-------------|
| 200 | OK - Request successful |
| 201 | Created - Resource created successfully |
| 400 | Bad Request - Invalid request data |
| 401 | Unauthorized - Authentication required |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource not found |
| 409 | Conflict - Resource already exists |
| 422 | Unprocessable Entity - Validation failed |
| 500 | Internal Server Error - Server error |

---

## Health Check

### Get Server Health
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "1.0.0",
  "services": {
    "database": "healthy",
    "redis": "healthy",
    "firebase": "healthy",
    "fcm": "healthy"
  },
  "uptime": 3600
}
```

---

## Order Management

### Create Order
```http
POST /orders
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "restaurant_id": "uuid",
  "items": [
    {
      "menu_item_id": "uuid",
      "quantity": 2,
      "special_instructions": "Extra spicy",
      "customizations": [
        {
          "option_id": "uuid",
          "choice_id": "uuid"
        }
      ]
    }
  ],
  "delivery_address": {
    "street": "123 Main St",
    "city": "Mumbai",
    "state": "Maharashtra",
    "postal_code": "400001",
    "country": "India",
    "latitude": 19.0760,
    "longitude": 72.8777,
    "landmark": "Near Central Mall"
  },
  "payment_method": {
    "type": "UPI",
    "vpa": "user@paytm"
  },
  "delivery_instructions": "Ring the bell twice",
  "scheduled_delivery": "2024-01-15T12:00:00Z"
}
```

**Response (201 Created):**
```json
{
  "data": {
    "id": "uuid",
    "order_number": "ORD123456",
    "status": "placed",
    "total_amount": 450.00,
    "tax_amount": 45.00,
    "delivery_fee": 30.00,
    "estimated_delivery_time": "2024-01-15T11:30:00Z",
    "tracking_id": "DEL123456",
    "payment_status": "pending",
    "created_at": "2024-01-15T10:00:00Z"
  }
}
```

### Get Order Details
```http
GET /orders/{order_id}
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "data": {
    "id": "uuid",
    "order_number": "ORD123456",
    "customer_id": "uuid",
    "restaurant_id": "uuid",
    "delivery_person_id": "uuid",
    "status": "in_transit",
    "items": [
      {
        "menu_item_id": "uuid",
        "name": "Butter Chicken",
        "quantity": 2,
        "unit_price": 180.00,
        "total_price": 360.00,
        "special_instructions": "Extra spicy"
      }
    ],
    "total_amount": 450.00,
    "tax_amount": 45.00,
    "delivery_fee": 30.00,
    "delivery_address": { ... },
    "restaurant_address": { ... },
    "created_at": "2024-01-15T10:00:00Z",
    "estimated_delivery_time": "2024-01-15T11:30:00Z",
    "tracking": {
      "current_location": {
        "latitude": 19.0760,
        "longitude": 72.8777
      },
      "estimated_arrival": "2024-01-15T11:25:00Z",
      "distance_remaining": 2.5
    },
    "timeline": [
      {
        "status": "placed",
        "timestamp": "2024-01-15T10:00:00Z",
        "message": "Order placed successfully"
      },
      {
        "status": "confirmed",
        "timestamp": "2024-01-15T10:02:00Z",
        "message": "Restaurant confirmed your order"
      }
    ]
  }
}
```

### Update Order Status
```http
PUT /orders/{order_id}/status
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "status": "confirmed",
  "estimated_preparation_time": 20,
  "notes": "Order confirmed, preparing now"
}
```

**Response (200 OK):**
```json
{
  "data": {
    "id": "uuid",
    "status": "confirmed",
    "estimated_delivery_time": "2024-01-15T11:30:00Z",
    "updated_at": "2024-01-15T10:02:00Z"
  }
}
```

### List Orders
```http
GET /orders?status=active&limit=20&offset=0
Authorization: Bearer <token>
```

**Query Parameters:**
- `status`: Filter by order status (active, completed, cancelled)
- `restaurant_id`: Filter by restaurant (restaurant users only)
- `delivery_person_id`: Filter by delivery person
- `date_from`: Start date filter (ISO 8601)
- `date_to`: End date filter (ISO 8601)
- `limit`: Number of results (default: 20, max: 100)
- `offset`: Pagination offset (default: 0)

**Response (200 OK):**
```json
{
  "data": {
    "orders": [ ... ],
    "total_count": 150,
    "has_more": true
  }
}
```

### Cancel Order
```http
DELETE /orders/{order_id}
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "reason": "customer_request",
  "notes": "Customer changed mind"
}
```

---

## Payment Management

### Process Payment
```http
POST /payments
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "order_id": "uuid",
  "payment_method": {
    "type": "UPI",
    "vpa": "user@paytm"
  },
  "amount": 450.00
}
```

**Response (201 Created):**
```json
{
  "data": {
    "id": "uuid",
    "status": "processing",
    "transaction_id": "TXN123456789",
    "payment_url": "upi://pay?pa=merchant@upi&pn=Restaurant&am=450.00&tr=TXN123456789",
    "expires_at": "2024-01-15T10:15:00Z",
    "created_at": "2024-01-15T10:00:00Z"
  }
}
```

### Get Payment Status
```http
GET /payments/{payment_id}
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "data": {
    "id": "uuid",
    "order_id": "uuid",
    "amount": 450.00,
    "currency": "INR",
    "status": "completed",
    "payment_method": {
      "type": "UPI",
      "vpa": "user@paytm"
    },
    "transaction_id": "TXN123456789",
    "gateway_response": {
      "reference_id": "GTW789456123",
      "status": "SUCCESS"
    },
    "created_at": "2024-01-15T10:00:00Z",
    "completed_at": "2024-01-15T10:01:30Z"
  }
}
```

### Process Refund
```http
POST /payments/{payment_id}/refund
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "amount": 450.00,
  "reason": "order_cancelled",
  "notes": "Order cancelled by restaurant"
}
```

---

## User Management

### Get User Profile
```http
GET /users/profile
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
{
  "data": {
    "id": "uuid",
    "firebase_uid": "firebase_user_id",
    "email": "user@example.com",
    "phone": "+91-9876543210",
    "email_verified": true,
    "phone_verified": true,
    "role": "customer",
    "profile": {
      "name": "John Doe",
      "avatar_url": "https://...",
      "date_of_birth": "1990-01-15",
      "gender": "male"
    },
    "addresses": [
      {
        "id": "uuid",
        "type": "home",
        "street": "123 Main St",
        "city": "Mumbai",
        "state": "Maharashtra",
        "postal_code": "400001",
        "is_default": true
      }
    ],
    "preferences": {
      "cuisine_types": ["Indian", "Chinese"],
      "dietary_restrictions": ["vegetarian"],
      "notification_settings": {
        "order_updates": true,
        "promotional": false
      }
    },
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
}
```

### Update User Profile
```http
PUT /users/profile
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "profile": {
    "name": "John Doe",
    "date_of_birth": "1990-01-15",
    "gender": "male"
  },
  "preferences": {
    "cuisine_types": ["Indian", "Chinese"],
    "dietary_restrictions": ["vegetarian"],
    "notification_settings": {
      "order_updates": true,
      "promotional": false
    }
  }
}
```

### Add User Address
```http
POST /users/addresses
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "type": "work",
  "street": "456 Business Park",
  "city": "Mumbai",
  "state": "Maharashtra",
  "postal_code": "400001",
  "country": "India",
  "latitude": 19.0760,
  "longitude": 72.8777,
  "landmark": "Near Tech Tower",
  "is_default": false
}
```

---

## Restaurant Management

### Register Restaurant
```http
POST /restaurants
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "name": "Delicious Bites",
  "description": "Authentic Indian cuisine with modern twist",
  "address": {
    "street": "789 Food Street",
    "city": "Mumbai",
    "state": "Maharashtra",
    "postal_code": "400001",
    "country": "India",
    "latitude": 19.0760,
    "longitude": 72.8777
  },
  "cuisine_types": ["Indian", "North Indian", "Vegetarian"],
  "operating_hours": {
    "monday": {"open": "09:00", "close": "22:00"},
    "tuesday": {"open": "09:00", "close": "22:00"},
    "wednesday": {"open": "09:00", "close": "22:00"},
    "thursday": {"open": "09:00", "close": "22:00"},
    "friday": {"open": "09:00", "close": "23:00"},
    "saturday": {"open": "09:00", "close": "23:00"},
    "sunday": {"open": "10:00", "close": "22:00"}
  },
  "contact": {
    "phone": "+91-9876543210",
    "email": "restaurant@example.com",
    "website": "https://restaurant.com"
  },
  "delivery_settings": {
    "delivery_radius": 5.0,
    "minimum_order_amount": 200.00,
    "delivery_fee": 30.00,
    "estimated_preparation_time": 30
  }
}
```

### Get Restaurant Details
```http
GET /restaurants/{restaurant_id}
Authorization: Bearer <token>
```

### Update Restaurant Menu
```http
PUT /restaurants/{restaurant_id}/menu
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "categories": [
    {
      "id": "uuid",
      "name": "Main Course",
      "description": "Hearty main dishes",
      "display_order": 1,
      "items": [
        {
          "id": "uuid",
          "name": "Butter Chicken",
          "description": "Creamy tomato-based curry with tender chicken",
          "price": 280.00,
          "image_url": "https://...",
          "available": true,
          "preparation_time": 15,
          "dietary_info": ["non-vegetarian", "contains-dairy"],
          "spice_level": "medium",
          "customizations": [
            {
              "name": "Spice Level",
              "required": false,
              "options": [
                {"id": "uuid", "name": "Mild", "price": 0.00},
                {"id": "uuid", "name": "Medium", "price": 0.00},
                {"id": "uuid", "name": "Hot", "price": 0.00}
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

### Set Restaurant Availability
```http
PUT /restaurants/{restaurant_id}/availability
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "available": true,
  "reason": "operational",
  "estimated_reopening": null
}
```

---

## Delivery Management

### Register Delivery Person
```http
POST /delivery-persons
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "vehicle_type": "motorcycle",
  "license_number": "MH12AB1234",
  "vehicle_details": {
    "make": "Honda",
    "model": "Activa",
    "year": 2022,
    "color": "Red"
  },
  "documents": {
    "driving_license": "document_url",
    "vehicle_registration": "document_url",
    "insurance": "document_url",
    "identity_proof": "document_url"
  },
  "bank_details": {
    "account_number": "1234567890",
    "ifsc_code": "HDFC0001234",
    "account_holder_name": "John Doe",
    "bank_name": "HDFC Bank"
  },
  "emergency_contact": {
    "name": "Jane Doe",
    "phone": "+91-9876543211",
    "relationship": "spouse"
  }
}
```

### Update Delivery Person Location
```http
PUT /delivery-persons/location
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "latitude": 19.0760,
  "longitude": 72.8777,
  "accuracy": 5.0,
  "heading": 45.0,
  "speed": 25.5,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Get Delivery Assignments
```http
GET /delivery-persons/assignments?status=active
Authorization: Bearer <token>
```

### Accept Delivery Assignment
```http
POST /delivery-persons/assignments/{assignment_id}/accept
Authorization: Bearer <token>
```

### Complete Delivery
```http
POST /delivery-persons/assignments/{assignment_id}/complete
Content-Type: application/json
Authorization: Bearer <token>
```

**Request Body:**
```json
{
  "proof_of_delivery": {
    "type": "photo",
    "url": "https://...",
    "notes": "Delivered to customer at door"
  },
  "customer_rating": 5,
  "delivery_notes": "Smooth delivery"
}
```

---

## Real-time Updates (WebSocket)

### Connection
```javascript
const ws = new WebSocket('wss://api.delivery-platform.com/ws');

ws.onopen = () => {
  // Authenticate connection
  ws.send(JSON.stringify({
    type: 'authenticate',
    token: 'firebase-jwt-token'
  }));
};
```

### Subscribe to Order Updates
```javascript
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'order_updates',
  order_id: 'uuid'
}));
```

### Message Types

#### Order Status Update
```json
{
  "type": "order_status_update",
  "data": {
    "order_id": "uuid",
    "status": "confirmed",
    "estimated_delivery_time": "2024-01-15T11:30:00Z",
    "message": "Restaurant confirmed your order"
  }
}
```

#### Location Update
```json
{
  "type": "location_update",
  "data": {
    "order_id": "uuid",
    "delivery_person_id": "uuid",
    "location": {
      "latitude": 19.0760,
      "longitude": 72.8777
    },
    "estimated_arrival": "2024-01-15T11:25:00Z"
  }
}
```

#### Notification
```json
{
  "type": "notification",
  "data": {
    "title": "Order Update",
    "message": "Your order is ready for pickup",
    "type": "order_ready",
    "order_id": "uuid"
  }
}
```

---

## Error Codes

| Code | Description |
|------|-------------|
| `AUTH_REQUIRED` | Authentication token required |
| `AUTH_INVALID` | Invalid authentication token |
| `AUTH_EXPIRED` | Authentication token expired |
| `EMAIL_NOT_VERIFIED` | Email verification required |
| `PHONE_NOT_VERIFIED` | Phone verification required |
| `INSUFFICIENT_PERMISSIONS` | User lacks required permissions |
| `VALIDATION_FAILED` | Request validation failed |
| `RESOURCE_NOT_FOUND` | Requested resource not found |
| `RESOURCE_CONFLICT` | Resource already exists |
| `ORDER_NOT_FOUND` | Order not found |
| `ORDER_INVALID_STATUS` | Invalid order status transition |
| `PAYMENT_FAILED` | Payment processing failed |
| `RESTAURANT_UNAVAILABLE` | Restaurant not accepting orders |
| `DELIVERY_UNAVAILABLE` | No delivery persons available |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `INTERNAL_ERROR` | Internal server error |

---

## Rate Limits

| Endpoint Category | Limit | Window |
|-------------------|-------|--------|
| Authentication | 10 requests | 1 minute |
| Order Operations | 100 requests | 1 minute |
| Payment Operations | 50 requests | 1 minute |
| Profile Updates | 20 requests | 1 minute |
| Location Updates | 200 requests | 1 minute |
| General API | 1000 requests | 1 minute |

Rate limit headers are included in all responses:
- `X-RateLimit-Limit`: Request limit per window
- `X-RateLimit-Remaining`: Remaining requests in current window
- `X-RateLimit-Reset`: Time when the rate limit resets

---

## SDKs and Libraries

### JavaScript/TypeScript
```bash
npm install @delivery-platform/sdk
```

### Python
```bash
pip install delivery-platform-sdk
```

### Java
```xml
<dependency>
    <groupId>com.delivery-platform</groupId>
    <artifactId>delivery-platform-sdk</artifactId>
    <version>1.0.0</version>
</dependency>
```

---

*For more detailed examples and advanced usage, see our [Complete Documentation](./COMPREHENSIVE_DOCUMENTATION.md).*