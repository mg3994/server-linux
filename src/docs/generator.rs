use crate::docs::models::*;
use serde_json::json;
use std::collections::HashMap;

pub struct ApiDocGenerator;

impl ApiDocGenerator {
    pub fn generate_full_documentation() -> ApiDocumentation {
        ApiDocumentation {
            openapi: "3.0.3".to_string(),
            info: Self::generate_api_info(),
            servers: Self::generate_servers(),
            paths: Self::generate_all_paths(),
            components: Self::generate_components(),
            tags: Self::generate_tags(),
        }
    }

    fn generate_api_info() -> ApiInfo {
        ApiInfo {
            title: "Multi-Vendor Food Delivery API".to_string(),
            description: "Comprehensive API for managing restaurants, orders, delivery, and payments in a multi-vendor food delivery platform optimized for the Indian market.".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            contact: Some(Contact {
                name: "API Support".to_string(),
                email: "support@fooddelivery.com".to_string(),
                url: "https://fooddelivery.com/support".to_string(),
            }),
            license: Some(License {
                name: "MIT".to_string(),
                url: "https://opensource.org/licenses/MIT".to_string(),
            }),
        }
    }

    fn generate_servers() -> Vec<ServerInfo> {
        vec![
            ServerInfo {
                url: "https://api.fooddelivery.com/v1".to_string(),
                description: "Production server".to_string(),
            },
            ServerInfo {
                url: "https://staging-api.fooddelivery.com/v1".to_string(),
                description: "Staging server".to_string(),
            },
            ServerInfo {
                url: "http://localhost:3000".to_string(),
                description: "Development server".to_string(),
            },
        ]
    }

    fn generate_all_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        // Health endpoints
        paths.extend(Self::generate_health_paths());
        
        // Restaurant endpoints
        paths.extend(Self::generate_restaurant_paths());
        
        // Order endpoints
        paths.extend(Self::generate_order_paths());
        
        // Delivery endpoints
        paths.extend(Self::generate_delivery_paths());
        
        // Payment endpoints
        paths.extend(Self::generate_payment_paths());
        
        // Analytics endpoints
        paths.extend(Self::generate_analytics_paths());
        
        // India-specific endpoints
        paths.extend(Self::generate_india_paths());

        paths
    }

    fn generate_health_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/health".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Health".to_string()],
                summary: "Basic health check".to_string(),
                description: "Returns OK if the service is running".to_string(),
                operation_id: "healthCheck".to_string(),
                parameters: vec![],
                request_body: None,
                responses: Self::generate_simple_responses(),
                security: vec![],
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        });

        paths.insert("/health/detailed".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Health".to_string()],
                summary: "Detailed health check".to_string(),
                description: "Returns detailed health information including component status".to_string(),
                operation_id: "detailedHealthCheck".to_string(),
                parameters: vec![],
                request_body: None,
                responses: Self::generate_health_responses(),
                security: vec![],
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_restaurant_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/restaurants".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Restaurants".to_string()],
                summary: "List restaurants".to_string(),
                description: "Get a list of restaurants with optional filtering".to_string(),
                operation_id: "listRestaurants".to_string(),
                parameters: vec![
                    Parameter {
                        name: "city".to_string(),
                        r#in: ParameterLocation::Query,
                        description: "Filter by city".to_string(),
                        required: false,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: None,
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!("Mumbai")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("Mumbai")),
                    },
                    Parameter {
                        name: "cuisine".to_string(),
                        r#in: ParameterLocation::Query,
                        description: "Filter by cuisine type".to_string(),
                        required: false,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: None,
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: Some(vec![
                                json!("North Indian"),
                                json!("South Indian"),
                                json!("Chinese"),
                                json!("Italian"),
                                json!("Fast Food"),
                            ]),
                            example: Some(json!("North Indian")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("North Indian")),
                    },
                ],
                request_body: None,
                responses: Self::generate_restaurant_list_responses(),
                security: vec![],
            }),
            post: Some(Operation {
                tags: vec!["Restaurants".to_string()],
                summary: "Create restaurant".to_string(),
                description: "Create a new restaurant (requires authentication)".to_string(),
                operation_id: "createRestaurant".to_string(),
                parameters: vec![],
                request_body: Some(RequestBody {
                    description: "Restaurant creation data".to_string(),
                    content: Self::generate_restaurant_request_content(),
                    required: true,
                }),
                responses: Self::generate_restaurant_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            put: None,
            delete: None,
            patch: None,
        });

        paths.insert("/restaurants/{id}".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Restaurants".to_string()],
                summary: "Get restaurant".to_string(),
                description: "Get detailed information about a specific restaurant".to_string(),
                operation_id: "getRestaurant".to_string(),
                parameters: vec![
                    Parameter {
                        name: "id".to_string(),
                        r#in: ParameterLocation::Path,
                        description: "Restaurant ID".to_string(),
                        required: true,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: Some("uuid".to_string()),
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                    },
                ],
                request_body: None,
                responses: Self::generate_restaurant_responses(),
                security: vec![],
            }),
            post: None,
            put: Some(Operation {
                tags: vec!["Restaurants".to_string()],
                summary: "Update restaurant".to_string(),
                description: "Update restaurant information (requires authentication)".to_string(),
                operation_id: "updateRestaurant".to_string(),
                parameters: vec![
                    Parameter {
                        name: "id".to_string(),
                        r#in: ParameterLocation::Path,
                        description: "Restaurant ID".to_string(),
                        required: true,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: Some("uuid".to_string()),
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                    },
                ],
                request_body: Some(RequestBody {
                    description: "Restaurant update data".to_string(),
                    content: Self::generate_restaurant_update_content(),
                    required: true,
                }),
                responses: Self::generate_restaurant_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            delete: Some(Operation {
                tags: vec!["Restaurants".to_string()],
                summary: "Delete restaurant".to_string(),
                description: "Delete a restaurant (requires authentication)".to_string(),
                operation_id: "deleteRestaurant".to_string(),
                parameters: vec![
                    Parameter {
                        name: "id".to_string(),
                        r#in: ParameterLocation::Path,
                        description: "Restaurant ID".to_string(),
                        required: true,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: Some("uuid".to_string()),
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("123e4567-e89b-12d3-a456-426614174000")),
                    },
                ],
                request_body: None,
                responses: Self::generate_delete_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            patch: None,
        });

        paths
    }

    fn generate_order_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/orders".to_string(), PathItem {
            get: None,
            post: Some(Operation {
                tags: vec!["Orders".to_string()],
                summary: "Create order".to_string(),
                description: "Create a new food order".to_string(),
                operation_id: "createOrder".to_string(),
                parameters: vec![],
                request_body: Some(RequestBody {
                    description: "Order creation data".to_string(),
                    content: Self::generate_order_request_content(),
                    required: true,
                }),
                responses: Self::generate_order_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_delivery_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/delivery/nearby".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Delivery".to_string()],
                summary: "Find nearby delivery persons".to_string(),
                description: "Get list of available delivery persons near a location".to_string(),
                operation_id: "getNearbyDeliveryPersons".to_string(),
                parameters: vec![
                    Parameter {
                        name: "latitude".to_string(),
                        r#in: ParameterLocation::Query,
                        description: "Latitude coordinate".to_string(),
                        required: true,
                        schema: Schema {
                            r#type: Some("number".to_string()),
                            format: Some("double".to_string()),
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!(19.0760)),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!(19.0760)),
                    },
                    Parameter {
                        name: "longitude".to_string(),
                        r#in: ParameterLocation::Query,
                        description: "Longitude coordinate".to_string(),
                        required: true,
                        schema: Schema {
                            r#type: Some("number".to_string()),
                            format: Some("double".to_string()),
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: None,
                            example: Some(json!(72.8777)),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!(72.8777)),
                    },
                ],
                request_body: None,
                responses: Self::generate_delivery_list_responses(),
                security: vec![],
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_payment_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/payments".to_string(), PathItem {
            get: None,
            post: Some(Operation {
                tags: vec!["Payments".to_string()],
                summary: "Create payment".to_string(),
                description: "Process a payment for an order".to_string(),
                operation_id: "createPayment".to_string(),
                parameters: vec![],
                request_body: Some(RequestBody {
                    description: "Payment data".to_string(),
                    content: Self::generate_payment_request_content(),
                    required: true,
                }),
                responses: Self::generate_payment_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_analytics_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/analytics/business".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["Analytics".to_string()],
                summary: "Get business analytics".to_string(),
                description: "Get comprehensive business analytics data (admin only)".to_string(),
                operation_id: "getBusinessAnalytics".to_string(),
                parameters: vec![
                    Parameter {
                        name: "period".to_string(),
                        r#in: ParameterLocation::Query,
                        description: "Time period for analytics".to_string(),
                        required: false,
                        schema: Schema {
                            r#type: Some("string".to_string()),
                            format: None,
                            items: None,
                            properties: None,
                            required: None,
                            r#enum: Some(vec![
                                json!("today"),
                                json!("yesterday"),
                                json!("this_week"),
                                json!("last_week"),
                                json!("this_month"),
                                json!("last_month"),
                            ]),
                            example: Some(json!("this_week")),
                            description: None,
                            r#ref: None,
                        },
                        example: Some(json!("this_week")),
                    },
                ],
                request_body: None,
                responses: Self::generate_analytics_responses(),
                security: vec![Self::generate_bearer_auth()],
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_india_paths() -> HashMap<String, PathItem> {
        let mut paths = HashMap::new();

        paths.insert("/india/cities".to_string(), PathItem {
            get: Some(Operation {
                tags: vec!["India".to_string()],
                summary: "Get supported cities".to_string(),
                description: "Get list of cities where delivery is available in India".to_string(),
                operation_id: "getSupportedCities".to_string(),
                parameters: vec![],
                request_body: None,
                responses: Self::generate_cities_responses(),
                security: vec![],
            }),
            post: None,
            put: None,
            delete: None,
            patch: None,
        });

        paths
    }

    fn generate_components() -> Components {
        let mut schemas = HashMap::new();
        
        // Add common schemas
        schemas.insert("Restaurant".to_string(), Self::generate_restaurant_schema());
        schemas.insert("Order".to_string(), Self::generate_order_schema());
        schemas.insert("DeliveryPerson".to_string(), Self::generate_delivery_person_schema());
        schemas.insert("Payment".to_string(), Self::generate_payment_schema());
        schemas.insert("Error".to_string(), Self::generate_error_schema());

        let mut security_schemes = HashMap::new();
        security_schemes.insert("BearerAuth".to_string(), SecurityScheme {
            r#type: "http".to_string(),
            scheme: Some("bearer".to_string()),
            bearer_format: Some("JWT".to_string()),
            description: Some("Firebase JWT token".to_string()),
        });

        Components {
            schemas,
            security_schemes,
        }
    }

    fn generate_tags() -> Vec<Tag> {
        vec![
            Tag {
                name: "Health".to_string(),
                description: "Health check endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "Restaurants".to_string(),
                description: "Restaurant management endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "Orders".to_string(),
                description: "Order management endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "Delivery".to_string(),
                description: "Delivery management endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "Payments".to_string(),
                description: "Payment processing endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "Analytics".to_string(),
                description: "Analytics and reporting endpoints".to_string(),
                external_docs: None,
            },
            Tag {
                name: "India".to_string(),
                description: "India-specific configuration endpoints".to_string(),
                external_docs: None,
            },
        ]
    }

    // Helper methods for generating specific response types
    fn generate_simple_responses() -> HashMap<String, Response> {
        let mut responses = HashMap::new();
        responses.insert("200".to_string(), Response {
            description: "Success".to_string(),
            content: Some({
                let mut content = HashMap::new();
                content.insert("text/plain".to_string(), MediaType {
                    schema: Schema {
                        r#type: Some("string".to_string()),
                        format: None,
                        items: None,
                        properties: None,
                        required: None,
                        r#enum: None,
                        example: Some(json!("OK")),
                        description: None,
                        r#ref: None,
                    },
                    example: Some(json!("OK")),
                    examples: None,
                });
                content
            }),
            headers: None,
        });
        responses
    }

    fn generate_health_responses() -> HashMap<String, Response> {
        let mut responses = HashMap::new();
        responses.insert("200".to_string(), Response {
            description: "Health status".to_string(),
            content: Some({
                let mut content = HashMap::new();
                content.insert("application/json".to_string(), MediaType {
                    schema: Schema {
                        r#type: Some("object".to_string()),
                        format: None,
                        items: None,
                        properties: None,
                        required: None,
                        r#enum: None,
                        example: Some(json!({
                            "status": "healthy",
                            "timestamp": "2024-01-01T00:00:00Z",
                            "version": "1.0.0",
                            "uptime": 3600,
                            "components": {
                                "database": {
                                    "status": "healthy",
                                    "response_time": 25
                                }
                            }
                        })),
                        description: None,
                        r#ref: None,
                    },
                    example: None,
                    examples: None,
                });
                content
            }),
            headers: None,
        });
        responses
    }

    // Additional helper methods would be implemented here...
    fn generate_restaurant_list_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_restaurant_request_content() -> HashMap<String, MediaType> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_restaurant_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_restaurant_update_content() -> HashMap<String, MediaType> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_delete_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_order_request_content() -> HashMap<String, MediaType> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_order_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_delivery_list_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_payment_request_content() -> HashMap<String, MediaType> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_payment_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_analytics_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_cities_responses() -> HashMap<String, Response> {
        HashMap::new() // Simplified for brevity
    }

    fn generate_bearer_auth() -> SecurityRequirement {
        let mut schemes = HashMap::new();
        schemes.insert("BearerAuth".to_string(), vec![]);
        SecurityRequirement { schemes }
    }

    // Schema generators
    fn generate_restaurant_schema() -> Schema {
        Schema {
            r#type: Some("object".to_string()),
            format: None,
            items: None,
            properties: Some({
                let mut props = HashMap::new();
                props.insert("id".to_string(), Schema {
                    r#type: Some("string".to_string()),
                    format: Some("uuid".to_string()),
                    items: None,
                    properties: None,
                    required: None,
                    r#enum: None,
                    example: None,
                    description: Some("Unique restaurant identifier".to_string()),
                    r#ref: None,
                });
                props.insert("name".to_string(), Schema {
                    r#type: Some("string".to_string()),
                    format: None,
                    items: None,
                    properties: None,
                    required: None,
                    r#enum: None,
                    example: Some(json!("Spice Garden")),
                    description: Some("Restaurant name".to_string()),
                    r#ref: None,
                });
                props
            }),
            required: Some(vec!["id".to_string(), "name".to_string()]),
            r#enum: None,
            example: None,
            description: Some("Restaurant information".to_string()),
            r#ref: None,
        }
    }

    fn generate_order_schema() -> Schema {
        Schema {
            r#type: Some("object".to_string()),
            format: None,
            items: None,
            properties: None,
            required: None,
            r#enum: None,
            example: None,
            description: Some("Order information".to_string()),
            r#ref: None,
        }
    }

    fn generate_delivery_person_schema() -> Schema {
        Schema {
            r#type: Some("object".to_string()),
            format: None,
            items: None,
            properties: None,
            required: None,
            r#enum: None,
            example: None,
            description: Some("Delivery person information".to_string()),
            r#ref: None,
        }
    }

    fn generate_payment_schema() -> Schema {
        Schema {
            r#type: Some("object".to_string()),
            format: None,
            items: None,
            properties: None,
            required: None,
            r#enum: None,
            example: None,
            description: Some("Payment information".to_string()),
            r#ref: None,
        }
    }

    fn generate_error_schema() -> Schema {
        Schema {
            r#type: Some("object".to_string()),
            format: None,
            items: None,
            properties: Some({
                let mut props = HashMap::new();
                props.insert("error".to_string(), Schema {
                    r#type: Some("string".to_string()),
                    format: None,
                    items: None,
                    properties: None,
                    required: None,
                    r#enum: None,
                    example: Some(json!("Invalid request")),
                    description: Some("Error message".to_string()),
                    r#ref: None,
                });
                props
            }),
            required: Some(vec!["error".to_string()]),
            r#enum: None,
            example: None,
            description: Some("Error response".to_string()),
            r#ref: None,
        }
    }
}