use crate::analytics::handlers::{
    export_analytics_report, get_business_analytics, get_predictive_analytics, 
    get_real_time_metrics, get_restaurant_analytics,
};
use crate::analytics::AnalyticsService;
use crate::auth::middleware::{auth_middleware, SharedFirebaseAuth};
use crate::database::Database;
use crate::delivery::handlers::{
    assign_order, calculate_delivery_time_estimate, deactivate_delivery_person,
    get_delivery_analytics, get_delivery_person, get_delivery_person_stats,
    get_india_delivery_zones, get_nearby_delivery_persons, register_delivery_person,
    update_delivery_person, update_delivery_status, update_location, verify_delivery_person,
};
use crate::delivery::websocket_handlers::{
    admin_websocket_handler, broadcast_test_message, customer_websocket_handler,
    delivery_person_websocket_handler, delivery_websocket_handler, get_websocket_stats,
    restaurant_websocket_handler,
};
use crate::delivery::enhanced_handlers::{
    assign_order_enhanced, batch_update_delivery_status, get_delivery_heatmap,
    get_delivery_person_status, get_delivery_tracking, get_live_delivery_metrics,
    get_real_time_delivery_analytics, handle_emergency_alert, update_delivery_status_enhanced,
    update_location_enhanced,
};
use crate::delivery::{DeliveryWebSocketManager, EnhancedDeliveryService};
use crate::india::handlers::{
    calculate_delivery_time, calculate_gst, get_cuisine_types, get_delivery_zones, get_gst_rates,
    get_india_config, get_indian_banks, get_indian_states, get_localization_config,
    get_payment_fees, get_supported_cities, get_upi_apps,
};
use crate::metrics::{health_detailed_handler, metrics_handler, MetricsCollector};
use crate::orders::handlers::{
    create_order, get_customer_orders, get_order, update_order_status, SharedFCMService,
};
use crate::payments::handlers::{create_payment, get_payment};
use crate::restaurants::handlers::{
    create_menu_item, create_restaurant, delete_menu_item, delete_restaurant, get_menu,
    get_restaurant, get_restaurant_orders, get_restaurants_by_city, get_restaurants_by_cuisine,
    list_restaurants, search_restaurants, update_menu_item, update_restaurant,
    update_restaurant_status,
};
use crate::websocket::WebSocketManager;
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pub fcm_service: SharedFCMService,
    pub database: Database,
    pub websocket_manager: WebSocketManager,
    pub delivery_websocket_manager: DeliveryWebSocketManager,
    pub enhanced_delivery_service: std::sync::Arc<EnhancedDeliveryService>,
    pub metrics: MetricsCollector,
    pub analytics_service: AnalyticsService,
}

pub fn create_routes(firebase_auth: SharedFirebaseAuth, app_state: AppState) -> Router {
    // Create routes with different states and then merge them
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/health/detailed", get(health_detailed_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(app_state.metrics.clone());

    let order_routes = Router::new()
        .route("/orders", post(create_order))
        .route("/orders/:id", get(get_order))
        .route("/orders/:id/status", put(update_order_status))
        .route("/customers/:id/orders", get(get_customer_orders))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.fcm_service.clone());

    let payment_routes = Router::new()
        .route("/payments", post(create_payment))
        .route("/payments/:id", get(get_payment))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.fcm_service.clone());

    // Restaurant routes (mixed public and authenticated)
    let restaurant_public_routes = Router::new()
        .route("/restaurants", get(list_restaurants))
        .route("/restaurants/search", get(search_restaurants))
        .route("/restaurants/:id", get(get_restaurant))
        .route("/restaurants/:id/menu", get(get_menu))
        .route("/restaurants/city/:city", get(get_restaurants_by_city))
        .route(
            "/restaurants/cuisine/:cuisine_type",
            get(get_restaurants_by_cuisine),
        )
        .with_state(app_state.clone());

    let restaurant_auth_routes = Router::new()
        .route("/restaurants", post(create_restaurant))
        .route("/restaurants/:id", put(update_restaurant))
        .route("/restaurants/:id", axum::routing::delete(delete_restaurant))
        .route("/restaurants/:id/status", put(update_restaurant_status))
        .route("/restaurants/:id/orders", get(get_restaurant_orders))
        .route("/restaurants/:id/menu", post(create_menu_item))
        .route("/restaurants/:id/menu/:item_id", put(update_menu_item))
        .route(
            "/restaurants/:id/menu/:item_id",
            axum::routing::delete(delete_menu_item),
        )
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.clone());

    // Delivery routes (mixed public and authenticated)
    let delivery_public_routes = Router::new()
        .route("/delivery/nearby", get(get_nearby_delivery_persons))
        .route("/delivery/zones", get(get_india_delivery_zones))
        .route("/delivery/estimate-time", get(calculate_delivery_time_estimate))
        .route("/delivery/:id", get(get_delivery_person))
        .with_state(app_state.clone());

    let delivery_auth_routes = Router::new()
        .route("/delivery/register", post(register_delivery_person))
        .route("/delivery/:id", put(update_delivery_person))
        .route("/delivery/:id/location", put(update_location))
        .route("/delivery/:id/stats", get(get_delivery_person_stats))
        .route("/delivery/assign-order", post(assign_order))
        .route("/delivery/assignments/:id/status", put(update_delivery_status))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.clone());

    // WebSocket routes (authenticated)
    let websocket_routes = Router::new()
        .route("/ws/delivery", get(delivery_websocket_handler))
        .route("/ws/delivery-person/:id", get(delivery_person_websocket_handler))
        .route("/ws/restaurant/:id", get(restaurant_websocket_handler))
        .route("/ws/customer/:id", get(customer_websocket_handler))
        .route("/ws/admin", get(admin_websocket_handler))
        .route("/ws/stats", get(get_websocket_stats))
        .route("/ws/test-broadcast", post(broadcast_test_message))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.delivery_websocket_manager.clone());

    // Enhanced delivery routes with real-time features
    let enhanced_delivery_routes = Router::new()
        .route("/delivery/enhanced/:id/location", put(update_location_enhanced))
        .route("/delivery/enhanced/assignments/:id/status", put(update_delivery_status_enhanced))
        .route("/delivery/enhanced/assign-order", post(assign_order_enhanced))
        .route("/delivery/enhanced/:id/emergency", post(handle_emergency_alert))
        .route("/delivery/enhanced/tracking/:id", get(get_delivery_tracking))
        .route("/delivery/enhanced/analytics/real-time", get(get_real_time_delivery_analytics))
        .route("/delivery/enhanced/:id/status", get(get_delivery_person_status))
        .route("/delivery/enhanced/batch-update", post(batch_update_delivery_status))
        .route("/delivery/enhanced/heatmap", get(get_delivery_heatmap))
        .route("/delivery/enhanced/metrics/live", get(get_live_delivery_metrics))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.enhanced_delivery_service.clone());

    let delivery_admin_routes = Router::new()
        .route("/admin/delivery/:id/verify", put(verify_delivery_person))
        .route("/admin/delivery/:id/deactivate", put(deactivate_delivery_person))
        .route("/admin/delivery/analytics", get(get_delivery_analytics))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.clone());

    // Analytics routes (authenticated)
    let analytics_routes = Router::new()
        .route("/analytics/business", get(get_business_analytics))
        .route("/analytics/real-time", get(get_real_time_metrics))
        .route("/analytics/predictive", get(get_predictive_analytics))
        .route("/analytics/restaurant", get(get_restaurant_analytics))
        .route("/analytics/export", get(export_analytics_report))
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        .with_state(app_state.analytics_service.clone());

    // India-specific routes (public)
    let india_routes = Router::new()
        .route("/india/cities", get(get_supported_cities))
        .route("/india/states", get(get_indian_states))
        .route("/india/cuisines", get(get_cuisine_types))
        .route("/india/gst-rates", get(get_gst_rates))
        .route("/india/calculate-gst", get(calculate_gst))
        .route("/india/upi-apps", get(get_upi_apps))
        .route("/india/banks", get(get_indian_banks))
        .route("/india/payment-fees", get(get_payment_fees))
        .route("/india/delivery-zones", get(get_delivery_zones))
        .route("/india/config", get(get_india_config))
        .route("/india/localization", get(get_localization_config))
        .route("/india/delivery-time", get(calculate_delivery_time))
        .with_state(app_state.metrics.clone());

    // Merge all routes
    public_routes
        .merge(order_routes)
        .merge(payment_routes)
        .merge(restaurant_public_routes)
        .merge(restaurant_auth_routes)
        .merge(delivery_public_routes)
        .merge(delivery_auth_routes)
        .merge(delivery_admin_routes)
        .merge(websocket_routes)
        .merge(enhanced_delivery_routes)
        .merge(analytics_routes)
        .merge(india_routes)
}

async fn health_check() -> &'static str {
    "OK"
}
