# Delivery Management API Documentation

## Overview

The Delivery Management API provides comprehensive functionality for managing delivery personnel, order assignments, real-time tracking, and India-specific delivery features.

## Base URL

```
https://api.yourdeliveryapp.com/api
```

## Authentication

All authenticated endpoints require a Bearer token in the Authorization header:

```
Authorization: Bearer <firebase_jwt_token>
```

## Delivery Person Management

### Register Delivery Person

Register a new delivery person in the system.

**Endpoint:** `POST /delivery/register`  
**Authentication:** Required (User must have delivery_person role)

**Request Body:**
```json
{
  "name": "Rajesh Kumar",
  "phone": "+91-9876543210",
  "email": "rajesh.delivery@example.com",
  "vehicle_type": "motorcycle",
  "vehicle_number": "MH01AB1234",
  "license_number": "MH0120230001234",
  "aadhar_number": "123456789012",
  "pan_number": "ABCDE1234F",
  "bank_account_number": "1234567890123456",
  "ifsc_code": "SBIN0001234"
}
```

**Vehicle Types:**
- `bicycle`
- `motorcycle`
- `scooter`
- `car`
- `van`

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "user_id": "456e7890-e89b-12d3-a456-426614174001",
  "name": "Rajesh Kumar",
  "phone": "+91-9876543210",
  "email": "rajesh.delivery@example.com",
  "vehicle_type": "motorcycle",
  "vehicle_number": "MH01AB1234",
  "current_latitude": null,
  "current_longitude": null,
  "is_available": false,
  "is_verified": false,
  "rating": 0.0,
  "total_deliveries": 0,
  "successful_deliveries": 0,
  "average_delivery_time": null,
  "earnings_today": 0.0,
  "created_at": "2024-01-16T10:30:00Z"
}
```

### Get Delivery Person Details

Get details of a specific delivery person.

**Endpoint:** `GET /delivery/{delivery_person_id}`  
**Authentication:** Not required (public information)

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Rajesh Kumar",
  "vehicle_type": "motorcycle",
  "rating": 4.5,
  "total_deliveries": 150,
  "is_available": true,
  "is_verified": true
}
```

### Update Delivery Person

Update delivery person information.

**Endpoint:** `PUT /delivery/{delivery_person_id}`  
**Authentication:** Required (Must be the delivery person or admin)

**Request Body:**
```json
{
  "name": "Rajesh Kumar Singh",
  "phone": "+91-9876543211",
  "vehicle_type": "car",
  "vehicle_number": "MH01CD5678",
  "is_available": true
}
```

### Update Location

Update delivery person's current location.

**Endpoint:** `PUT /delivery/{delivery_person_id}/location`  
**Authentication:** Required (Must be the delivery person)

**Request Body:**
```json
{
  "latitude": 19.0760,
  "longitude": 72.8777,
  "speed": 25.5,
  "heading": 180.0
}
```

**Response:**
```json
{
  "success": true,
  "message": "Location updated successfully"
}
```

### Get Delivery Person Statistics

Get performance statistics for a delivery person.

**Endpoint:** `GET /delivery/{delivery_person_id}/stats`  
**Authentication:** Required (Must be the delivery person or admin)

**Response:**
```json
{
  "total_deliveries": 150,
  "successful_deliveries": 145,
  "success_rate": 96.67,
  "average_delivery_time": 28,
  "earnings_today": 450.0,
  "earnings_this_week": 2800.0,
  "earnings_this_month": 12500.0,
  "rating": 4.5,
  "total_ratings": 120
}
```

## Order Assignment and Management

### Assign Order to Delivery Person

Assign an order to a delivery person.

**Endpoint:** `POST /delivery/assign-order`  
**Authentication:** Required (Restaurant or admin)

**Request Body:**
```json
{
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "preferred_delivery_person_id": "123e4567-e89b-12d3-a456-426614174000",
  "max_distance_km": 10.0
}
```

**Response:**
```json
{
  "id": "abc1234d-e89b-12d3-a456-426614174003",
  "order_id": "789e0123-e89b-12d3-a456-426614174002",
  "delivery_person_id": "123e4567-e89b-12d3-a456-426614174000",
  "restaurant_id": "def5678e-e89b-12d3-a456-426614174004",
  "customer_id": "ghi9012f-e89b-12d3-a456-426614174005",
  "pickup_address": {
    "street": "123 Restaurant St",
    "city": "Mumbai",
    "state": "Maharashtra",
    "postal_code": "400001",
    "latitude": 19.0760,
    "longitude": 72.8777
  },
  "delivery_address": {
    "street": "456 Customer Ave",
    "city": "Mumbai",
    "state": "Maharashtra", 
    "postal_code": "400002",
    "latitude": 19.0896,
    "longitude": 72.8656
  },
  "status": "assigned",
  "assigned_at": "2024-01-16T12:00:00Z",
  "estimated_pickup_time": "2024-01-16T12:15:00Z",
  "estimated_delivery_time": "2024-01-16T12:45:00Z",
  "delivery_fee": 30.0
}
```

### Update Delivery Status

Update the status of a delivery assignment.

**Endpoint:** `PUT /delivery/assignments/{assignment_id}/{delivery_person_id}/status`  
**Authentication:** Required (Must be the assigned delivery person)

**Request Body:**
```json
{
  "status": "pickedup",
  "notes": "Food picked up successfully",
  "proof_of_delivery": {
    "photo": "base64_encoded_image_data",
    "timestamp": "2024-01-16T12:20:00Z"
  }
}
```

**Delivery Status Values:**
- `assigned` - Order assigned to delivery person
- `accepted` - Delivery person accepted the order
- `enroutetorestaurant` - On the way to restaurant
- `arrivedatrestaurant` - Arrived at restaurant
- `pickedup` - Food picked up from restaurant
- `enroutetocustomer` - On the way to customer
- `arrivedatcustomer` - Arrived at customer location
- `delivered` - Order delivered successfully
- `cancelled` - Delivery cancelled
- `failed` - Delivery failed

**Response:**
```json
{
  "id": "abc1234d-e89b-12d3-a456-426614174003",
  "status": "pickedup",
  "picked_up_at": "2024-01-16T12:20:00Z",
  "delivery_notes": "Food picked up successfully",
  "updated_at": "2024-01-16T12:20:00Z"
}
```

### Get Delivery Assignments

Get all assignments for a delivery person.

**Endpoint:** `GET /delivery/{delivery_person_id}/assignments`  
**Authentication:** Required (Must be the delivery person or admin)

**Query Parameters:**
- `status` (optional): Filter by delivery status
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Items per page (default: 20)

**Response:**
```json
[
  {
    "id": "abc1234d-e89b-12d3-a456-426614174003",
    "order_id": "789e0123-e89b-12d3-a456-426614174002",
    "status": "enroutetocustomer",
    "assigned_at": "2024-01-16T12:00:00Z",
    "picked_up_at": "2024-01-16T12:20:00Z",
    "estimated_delivery_time": "2024-01-16T12:45:00Z",
    "delivery_fee": 30.0,
    "pickup_address": {...},
    "delivery_address": {...}
  }
]
```

## Discovery and Search

### Get Nearby Delivery Persons

Find delivery persons near a specific location.

**Endpoint:** `GET /delivery/nearby`  
**Authentication:** Not required

**Query Parameters:**
- `latitude` (required): Latitude coordinate
- `longitude` (required): Longitude coordinate  
- `radius_km` (optional): Search radius in kilometers (default: 10)

**Response:**
```json
[
  {
    "delivery_person": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Rajesh Kumar",
      "vehicle_type": "motorcycle",
      "rating": 4.5,
      "is_available": true
    },
    "distance_km": 2.5,
    "estimated_arrival_minutes": 8
  }
]
```

## India-Specific Features

### Get India Delivery Zones

Get delivery zones and their configurations for India.

**Endpoint:** `GET /delivery/zones`  
**Authentication:** Not required

**Response:**
```json
[
  {
    "zone_name": "Mumbai Metro",
    "cities": ["Mumbai", "Navi Mumbai", "Thane"],
    "base_delivery_time_minutes": 30,
    "peak_hour_surcharge": 15.0,
    "weekend_surcharge": 10.0,
    "festival_surcharge": 25.0,
    "minimum_order_amount": 150.0
  },
  {
    "zone_name": "Delhi NCR",
    "cities": ["Delhi", "Gurgaon", "Noida", "Faridabad"],
    "base_delivery_time_minutes": 35,
    "peak_hour_surcharge": 20.0,
    "weekend_surcharge": 15.0,
    "festival_surcharge": 30.0,
    "minimum_order_amount": 200.0
  }
]
```

### Calculate Delivery Time Estimate

Get estimated delivery time for a route.

**Endpoint:** `GET /delivery/estimate-time`  
**Authentication:** Not required

**Query Parameters:**
- `pickup_lat` (required): Pickup latitude
- `pickup_lng` (required): Pickup longitude
- `delivery_lat` (required): Delivery latitude
- `delivery_lng` (required): Delivery longitude
- `city` (required): City name for local factors

**Response:**
```json
{
  "base_time_minutes": 25,
  "traffic_delay_minutes": 5,
  "weather_delay_minutes": 0,
  "peak_hour_delay_minutes": 10,
  "total_estimated_minutes": 40,
  "confidence_level": 0.85
}
```

## Admin Endpoints

### Verify Delivery Person

Verify a delivery person (admin only).

**Endpoint:** `PUT /admin/delivery/{delivery_person_id}/verify`  
**Authentication:** Required (Admin role)

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "is_verified": true,
  "updated_at": "2024-01-16T14:30:00Z"
}
```

### Deactivate Delivery Person

Deactivate a delivery person (admin only).

**Endpoint:** `PUT /admin/delivery/{delivery_person_id}/deactivate`  
**Authentication:** Required (Admin role)

**Response:**
```json
{
  "success": true,
  "message": "Delivery person deactivated successfully"
}
```

### Get Delivery Analytics

Get system-wide delivery analytics (admin only).

**Endpoint:** `GET /admin/delivery/analytics`  
**Authentication:** Required (Admin role)

**Response:**
```json
{
  "total_delivery_persons": 150,
  "active_delivery_persons": 120,
  "total_deliveries_today": 450,
  "successful_deliveries_today": 425,
  "success_rate_today": 94.44,
  "availability_rate": 80.0
}
```

## Error Responses

All endpoints may return the following error responses:

### 400 Bad Request
```json
{
  "error": "ValidationError",
  "message": "Invalid vehicle type provided",
  "details": {
    "field": "vehicle_type",
    "allowed_values": ["bicycle", "motorcycle", "scooter", "car", "van"]
  }
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "message": "Authentication required"
}
```

### 403 Forbidden
```json
{
  "error": "Forbidden", 
  "message": "You don't have permission to access this resource"
}
```

### 404 Not Found
```json
{
  "error": "NotFound",
  "message": "Delivery person not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "InternalServerError",
  "message": "An unexpected error occurred"
}
```

## Rate Limiting

API endpoints are rate limited to prevent abuse:

- **Public endpoints**: 100 requests per minute per IP
- **Authenticated endpoints**: 1000 requests per minute per user
- **Admin endpoints**: 500 requests per minute per admin

Rate limit headers are included in responses:
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642348800
```

## WebSocket Events

Real-time updates are available via WebSocket connection at `/ws`:

### Location Updates
```json
{
  "type": "location_update",
  "delivery_person_id": "123e4567-e89b-12d3-a456-426614174000",
  "latitude": 19.0760,
  "longitude": 72.8777,
  "timestamp": "2024-01-16T12:30:00Z"
}
```

### Status Updates
```json
{
  "type": "status_update",
  "assignment_id": "abc1234d-e89b-12d3-a456-426614174003",
  "status": "enroutetocustomer",
  "estimated_arrival": "2024-01-16T12:45:00Z"
}
```

## SDKs and Libraries

Official SDKs are available for:
- JavaScript/TypeScript
- React Native
- Flutter
- Android (Java/Kotlin)
- iOS (Swift)

## Support

For API support, contact:
- Email: api-support@yourdeliveryapp.com
- Documentation: https://docs.yourdeliveryapp.com
- Status Page: https://status.yourdeliveryapp.com