use crate::analytics::{AnalyticsQuery, AnalyticsService, BusinessAnalytics, PredictiveAnalytics, RealTimeMetrics};
use crate::auth::models::User;
use crate::error::{AppError, Result};
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::Json,
};

pub async fn get_business_analytics(
    Extension(_user): Extension<User>,
    Query(query): Query<AnalyticsQuery>,
    State(analytics_service): State<AnalyticsService>,
) -> Result<Json<BusinessAnalytics>> {
    // Only allow admin users to access analytics
    if _user.role.to_string() != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let analytics = analytics_service.get_business_analytics(&query).await?;
    Ok(Json(analytics))
}

pub async fn get_real_time_metrics(
    Extension(_user): Extension<User>,
    State(analytics_service): State<AnalyticsService>,
) -> Result<Json<RealTimeMetrics>> {
    // Allow admin and restaurant users to access real-time metrics
    match _user.role.to_string().as_str() {
        "admin" | "restaurant" => {},
        _ => return Err(AppError::Forbidden("Admin or restaurant access required".to_string())),
    }

    let metrics = analytics_service.get_real_time_metrics().await?;
    Ok(Json(metrics))
}

pub async fn get_predictive_analytics(
    Extension(_user): Extension<User>,
    State(analytics_service): State<AnalyticsService>,
) -> Result<Json<PredictiveAnalytics>> {
    // Only allow admin users to access predictive analytics
    if _user.role.to_string() != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let analytics = analytics_service.get_predictive_analytics().await?;
    Ok(Json(analytics))
}

pub async fn get_restaurant_analytics(
    Extension(user): Extension<User>,
    Query(query): Query<AnalyticsQuery>,
    State(analytics_service): State<AnalyticsService>,
) -> Result<Json<BusinessAnalytics>> {
    // Restaurant users can only see their own analytics
    let restaurant_query = match user.role.to_string().as_str() {
        "admin" => query,
        "restaurant" => {
            // For restaurant users, filter to their restaurant only
            // This would require additional logic to map user to restaurant
            AnalyticsQuery {
                restaurant_id: Some(user.id), // Assuming user.id maps to restaurant
                ..query
            }
        },
        _ => return Err(AppError::Forbidden("Restaurant or admin access required".to_string())),
    };

    let analytics = analytics_service.get_business_analytics(&restaurant_query).await?;
    Ok(Json(analytics))
}

pub async fn export_analytics_report(
    Extension(_user): Extension<User>,
    Query(query): Query<AnalyticsQuery>,
    State(analytics_service): State<AnalyticsService>,
) -> Result<(StatusCode, String)> {
    // Only allow admin users to export reports
    if _user.role.to_string() != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let analytics = analytics_service.get_business_analytics(&query).await?;
    
    // Generate CSV report
    let csv_content = generate_csv_report(&analytics)?;
    
    Ok((StatusCode::OK, csv_content))
}

fn generate_csv_report(analytics: &BusinessAnalytics) -> Result<String> {
    let mut csv = String::new();
    
    // Header
    csv.push_str("Metric,Value\n");
    
    // Restaurant metrics
    csv.push_str(&format!("Total Restaurants,{}\n", analytics.restaurant_metrics.total_restaurants));
    csv.push_str(&format!("Active Restaurants,{}\n", analytics.restaurant_metrics.active_restaurants));
    csv.push_str(&format!("New Restaurants,{}\n", analytics.restaurant_metrics.new_restaurants));
    csv.push_str(&format!("Average Restaurant Rating,{:.2}\n", analytics.restaurant_metrics.average_rating));
    
    // Delivery metrics
    csv.push_str(&format!("Total Delivery Persons,{}\n", analytics.delivery_metrics.total_delivery_persons));
    csv.push_str(&format!("Active Delivery Persons,{}\n", analytics.delivery_metrics.active_delivery_persons));
    csv.push_str(&format!("Total Deliveries,{}\n", analytics.delivery_metrics.total_deliveries));
    csv.push_str(&format!("Successful Deliveries,{}\n", analytics.delivery_metrics.successful_deliveries));
    csv.push_str(&format!("Delivery Success Rate,{:.2}%\n", analytics.delivery_metrics.delivery_success_rate));
    csv.push_str(&format!("Average Delivery Time,{:.2} minutes\n", analytics.delivery_metrics.average_delivery_time / 60.0));
    
    // Order metrics
    csv.push_str(&format!("Total Orders,{}\n", analytics.order_metrics.total_orders));
    csv.push_str(&format!("Completed Orders,{}\n", analytics.order_metrics.completed_orders));
    csv.push_str(&format!("Cancelled Orders,{}\n", analytics.order_metrics.cancelled_orders));
    csv.push_str(&format!("Pending Orders,{}\n", analytics.order_metrics.pending_orders));
    csv.push_str(&format!("Average Order Value,₹{:.2}\n", analytics.order_metrics.average_order_value));
    csv.push_str(&format!("Order Completion Rate,{:.2}%\n", analytics.order_metrics.order_completion_rate));
    
    // Revenue metrics
    csv.push_str(&format!("Total Revenue,₹{:.2}\n", analytics.revenue_metrics.total_revenue));
    csv.push_str(&format!("Restaurant Revenue,₹{:.2}\n", analytics.revenue_metrics.restaurant_revenue));
    csv.push_str(&format!("Delivery Fees,₹{:.2}\n", analytics.revenue_metrics.delivery_fees));
    csv.push_str(&format!("Platform Commission,₹{:.2}\n", analytics.revenue_metrics.platform_commission));
    csv.push_str(&format!("Taxes Collected,₹{:.2}\n", analytics.revenue_metrics.taxes_collected));
    csv.push_str(&format!("Net Revenue,₹{:.2}\n", analytics.revenue_metrics.net_revenue));
    
    // Customer metrics
    csv.push_str(&format!("Total Customers,{}\n", analytics.customer_metrics.total_customers));
    csv.push_str(&format!("New Customers,{}\n", analytics.customer_metrics.new_customers));
    csv.push_str(&format!("Returning Customers,{}\n", analytics.customer_metrics.returning_customers));
    csv.push_str(&format!("Customer Retention Rate,{:.2}%\n", analytics.customer_metrics.customer_retention_rate));
    csv.push_str(&format!("Average Orders per Customer,{:.2}\n", analytics.customer_metrics.average_orders_per_customer));
    csv.push_str(&format!("Customer Lifetime Value,₹{:.2}\n", analytics.customer_metrics.customer_lifetime_value));
    
    // Performance metrics
    csv.push_str(&format!("Average Response Time,{:.2}ms\n", analytics.performance_metrics.average_response_time));
    csv.push_str(&format!("System Uptime,{:.2}%\n", analytics.performance_metrics.system_uptime));
    csv.push_str(&format!("Error Rate,{:.2}%\n", analytics.performance_metrics.error_rate));
    csv.push_str(&format!("API Calls per Minute,{:.2}\n", analytics.performance_metrics.api_calls_per_minute));
    csv.push_str(&format!("Cache Hit Rate,{:.2}%\n", analytics.performance_metrics.cache_hit_rate));
    
    Ok(csv)
}