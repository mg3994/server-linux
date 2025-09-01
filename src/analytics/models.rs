use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessAnalytics {
    pub period: AnalyticsPeriod,
    pub restaurant_metrics: RestaurantMetrics,
    pub delivery_metrics: DeliveryMetrics,
    pub order_metrics: OrderMetrics,
    pub revenue_metrics: RevenueMetrics,
    pub customer_metrics: CustomerMetrics,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnalyticsPeriod {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    Custom { start: DateTime<Utc>, end: DateTime<Utc> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestaurantMetrics {
    pub total_restaurants: i64,
    pub active_restaurants: i64,
    pub new_restaurants: i64,
    pub top_performing_restaurants: Vec<RestaurantPerformance>,
    pub average_rating: f64,
    pub total_menu_items: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryMetrics {
    pub total_delivery_persons: i64,
    pub active_delivery_persons: i64,
    pub new_delivery_persons: i64,
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub average_delivery_time: f64,
    pub average_rating: f64,
    pub top_performers: Vec<DeliveryPersonPerformance>,
    pub delivery_success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderMetrics {
    pub total_orders: i64,
    pub completed_orders: i64,
    pub cancelled_orders: i64,
    pub pending_orders: i64,
    pub average_order_value: f64,
    pub peak_hours: Vec<PeakHourData>,
    pub order_completion_rate: f64,
    pub average_preparation_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueMetrics {
    pub total_revenue: f64,
    pub restaurant_revenue: f64,
    pub delivery_fees: f64,
    pub platform_commission: f64,
    pub taxes_collected: f64,
    pub refunds_issued: f64,
    pub net_revenue: f64,
    pub revenue_by_city: Vec<CityRevenue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerMetrics {
    pub total_customers: i64,
    pub new_customers: i64,
    pub returning_customers: i64,
    pub customer_retention_rate: f64,
    pub average_orders_per_customer: f64,
    pub customer_lifetime_value: f64,
    pub top_customers: Vec<CustomerPerformance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_response_time: f64,
    pub system_uptime: f64,
    pub error_rate: f64,
    pub api_calls_per_minute: f64,
    pub database_performance: DatabasePerformance,
    pub cache_hit_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestaurantPerformance {
    pub restaurant_id: Uuid,
    pub restaurant_name: String,
    pub total_orders: i64,
    pub total_revenue: f64,
    pub average_rating: f64,
    pub completion_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPersonPerformance {
    pub delivery_person_id: Uuid,
    pub name: String,
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub average_delivery_time: f64,
    pub average_rating: f64,
    pub total_earnings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeakHourData {
    pub hour: i32,
    pub order_count: i64,
    pub average_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityRevenue {
    pub city: String,
    pub revenue: f64,
    pub order_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerPerformance {
    pub customer_id: Uuid,
    pub total_orders: i64,
    pub total_spent: f64,
    pub average_order_value: f64,
    pub last_order_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabasePerformance {
    pub average_query_time: f64,
    pub slow_queries: i64,
    pub connection_pool_usage: f64,
    pub active_connections: i32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub period: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub restaurant_id: Option<Uuid>,
    pub city: Option<String>,
    pub include_details: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RealTimeMetrics {
    pub active_orders: i64,
    pub active_delivery_persons: i64,
    pub orders_per_minute: f64,
    pub average_wait_time: f64,
    pub system_load: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PredictiveAnalytics {
    pub demand_forecast: Vec<DemandForecast>,
    pub revenue_projection: RevenueProjection,
    pub capacity_recommendations: CapacityRecommendations,
    pub market_insights: MarketInsights,
}

#[derive(Debug, Serialize)]
pub struct DemandForecast {
    pub time_slot: DateTime<Utc>,
    pub predicted_orders: i64,
    pub confidence_level: f64,
    pub factors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RevenueProjection {
    pub next_week: f64,
    pub next_month: f64,
    pub growth_rate: f64,
    pub seasonal_factors: Vec<SeasonalFactor>,
}

#[derive(Debug, Serialize)]
pub struct CapacityRecommendations {
    pub recommended_delivery_persons: i64,
    pub peak_hour_staffing: Vec<StaffingRecommendation>,
    pub restaurant_capacity_alerts: Vec<CapacityAlert>,
}

#[derive(Debug, Serialize)]
pub struct MarketInsights {
    pub trending_cuisines: Vec<CuisineTrend>,
    pub competitor_analysis: CompetitorAnalysis,
    pub customer_behavior_insights: Vec<BehaviorInsight>,
}

#[derive(Debug, Serialize)]
pub struct SeasonalFactor {
    pub factor: String,
    pub impact_percentage: f64,
    pub period: String,
}

#[derive(Debug, Serialize)]
pub struct StaffingRecommendation {
    pub hour: i32,
    pub recommended_count: i64,
    pub current_count: i64,
    pub efficiency_gain: f64,
}

#[derive(Debug, Serialize)]
pub struct CapacityAlert {
    pub restaurant_id: Uuid,
    pub restaurant_name: String,
    pub current_capacity: f64,
    pub recommended_action: String,
}

#[derive(Debug, Serialize)]
pub struct CuisineTrend {
    pub cuisine_type: String,
    pub growth_rate: f64,
    pub order_volume: i64,
    pub revenue_impact: f64,
}

#[derive(Debug, Serialize)]
pub struct CompetitorAnalysis {
    pub market_share: f64,
    pub pricing_comparison: f64,
    pub service_quality_comparison: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BehaviorInsight {
    pub insight: String,
    pub impact: String,
    pub recommendation: String,
    pub confidence: f64,
}