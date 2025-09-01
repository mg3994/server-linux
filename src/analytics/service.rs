use crate::analytics::models::*;
use crate::database::Database;
use crate::error::{AppError, Result};
use chrono::{DateTime, Datelike, Duration, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct AnalyticsService {
    database: Database,
}

impl AnalyticsService {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn get_business_analytics(&self, query: &AnalyticsQuery) -> Result<BusinessAnalytics> {
        let (start_date, end_date) = self.parse_date_range(query)?;

        let restaurant_metrics = self.get_restaurant_metrics(&start_date, &end_date, query.restaurant_id).await?;
        let delivery_metrics = self.get_delivery_metrics(&start_date, &end_date).await?;
        let order_metrics = self.get_order_metrics(&start_date, &end_date, query.restaurant_id).await?;
        let revenue_metrics = self.get_revenue_metrics(&start_date, &end_date, query.restaurant_id).await?;
        let customer_metrics = self.get_customer_metrics(&start_date, &end_date).await?;
        let performance_metrics = self.get_performance_metrics().await?;

        Ok(BusinessAnalytics {
            period: self.determine_period(query),
            restaurant_metrics,
            delivery_metrics,
            order_metrics,
            revenue_metrics,
            customer_metrics,
            performance_metrics,
        })
    }

    pub async fn get_real_time_metrics(&self) -> Result<RealTimeMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        // Get active orders count
        let active_orders: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM orders WHERE status IN ('placed', 'confirmed', 'preparing', 'ready', 'out_for_delivery')"
        )
        .fetch_one(&mut *conn)
        .await?;

        // Get active delivery persons
        let active_delivery_persons: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_persons WHERE is_active = true AND is_available = true"
        )
        .fetch_one(&mut *conn)
        .await?;

        // Calculate orders per minute (last hour)
        let orders_per_minute: f64 = sqlx::query_scalar(
            "SELECT COUNT(*)::float / 60.0 FROM orders WHERE created_at >= NOW() - INTERVAL '1 hour'"
        )
        .fetch_one(&mut *conn)
        .await?;

        // Calculate average wait time
        let average_wait_time: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at))) 
             FROM orders 
             WHERE status = 'delivered' AND updated_at >= NOW() - INTERVAL '1 hour'"
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(RealTimeMetrics {
            active_orders,
            active_delivery_persons,
            orders_per_minute,
            average_wait_time: average_wait_time.unwrap_or(0.0),
            system_load: self.calculate_system_load().await?,
            timestamp: Utc::now(),
        })
    }

    pub async fn get_predictive_analytics(&self) -> Result<PredictiveAnalytics> {
        let demand_forecast = self.generate_demand_forecast().await?;
        let revenue_projection = self.generate_revenue_projection().await?;
        let capacity_recommendations = self.generate_capacity_recommendations().await?;
        let market_insights = self.generate_market_insights().await?;

        Ok(PredictiveAnalytics {
            demand_forecast,
            revenue_projection,
            capacity_recommendations,
            market_insights,
        })
    }

    async fn get_restaurant_metrics(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
        restaurant_id: Option<Uuid>,
    ) -> Result<RestaurantMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        let restaurant_filter = if let Some(id) = restaurant_id {
            format!("WHERE id = '{}'", id)
        } else {
            String::new()
        };

        // Total restaurants
        let total_restaurants: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM restaurants {}", restaurant_filter
        ))
        .fetch_one(&mut *conn)
        .await?;

        // Active restaurants
        let restaurant_filter_clause = if restaurant_id.is_some() { 
            format!("AND {}", &restaurant_filter[6..]) 
        } else { 
            String::new() 
        };
        let active_restaurants: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM restaurants WHERE is_active = true {}", 
            restaurant_filter_clause
        ))
        .fetch_one(&mut *conn)
        .await?;

        // New restaurants in period
        let restaurant_filter_clause = if restaurant_id.is_some() { 
            format!("AND {}", &restaurant_filter[6..]) 
        } else { 
            String::new() 
        };
        let new_restaurants: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM restaurants WHERE created_at BETWEEN $1 AND $2 {}", 
            restaurant_filter_clause
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        // Top performing restaurants
        let top_performing_restaurants = self.get_top_restaurants(start_date, end_date, restaurant_id).await?;

        // Average rating
        let restaurant_filter_clause = if restaurant_id.is_some() { 
            format!("AND {}", &restaurant_filter[6..]) 
        } else { 
            String::new() 
        };
        let average_rating: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT AVG(rating) FROM restaurants WHERE rating IS NOT NULL {}", 
            restaurant_filter_clause
        ))
        .fetch_one(&mut *conn)
        .await?;

        // Total menu items
        let total_menu_items: i64 = if let Some(id) = restaurant_id {
            sqlx::query_scalar("SELECT COUNT(*) FROM menu_items WHERE restaurant_id = $1")
                .bind(id)
                .fetch_one(&mut *conn)
                .await?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM menu_items")
                .fetch_one(&mut *conn)
                .await?
        };

        Ok(RestaurantMetrics {
            total_restaurants,
            active_restaurants,
            new_restaurants,
            top_performing_restaurants,
            average_rating: average_rating.unwrap_or(0.0),
            total_menu_items,
        })
    }

    async fn get_delivery_metrics(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<DeliveryMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        let total_delivery_persons: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_persons"
        )
        .fetch_one(&mut *conn)
        .await?;

        let active_delivery_persons: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_persons WHERE is_active = true"
        )
        .fetch_one(&mut *conn)
        .await?;

        let new_delivery_persons: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_persons WHERE created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let total_deliveries: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_assignments WHERE created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let successful_deliveries: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM delivery_assignments 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let average_delivery_time: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(EXTRACT(EPOCH FROM (delivered_at - assigned_at))) 
             FROM delivery_assignments 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let average_rating: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(rating) FROM delivery_reviews dr
             JOIN delivery_assignments da ON dr.assignment_id = da.id
             WHERE da.created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let top_performers = self.get_top_delivery_performers(start_date, end_date).await?;

        let delivery_success_rate = if total_deliveries > 0 {
            successful_deliveries as f64 / total_deliveries as f64 * 100.0
        } else {
            0.0
        };

        Ok(DeliveryMetrics {
            total_delivery_persons,
            active_delivery_persons,
            new_delivery_persons,
            total_deliveries,
            successful_deliveries,
            average_delivery_time: average_delivery_time.unwrap_or(0.0),
            average_rating: average_rating.unwrap_or(0.0),
            top_performers,
            delivery_success_rate,
        })
    }

    async fn get_order_metrics(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
        restaurant_id: Option<Uuid>,
    ) -> Result<OrderMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        let restaurant_filter = if let Some(id) = restaurant_id {
            format!("AND restaurant_id = '{}'", id)
        } else {
            String::new()
        };

        let total_orders: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM orders WHERE created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let completed_orders: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM orders 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let cancelled_orders: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM orders 
             WHERE status = 'cancelled' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let restaurant_filter_clause = if restaurant_id.is_some() { 
            format!("AND {}", &restaurant_filter[4..]) 
        } else { 
            String::new() 
        };
        let pending_orders: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM orders 
             WHERE status IN ('placed', 'confirmed', 'preparing', 'ready', 'out_for_delivery') {}", 
            restaurant_filter_clause
        ))
        .fetch_one(&mut *conn)
        .await?;

        let average_order_value: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT AVG(total_amount) FROM orders WHERE created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let peak_hours = self.get_peak_hours(start_date, end_date, restaurant_id).await?;

        let order_completion_rate = if total_orders > 0 {
            completed_orders as f64 / total_orders as f64 * 100.0
        } else {
            0.0
        };

        let average_preparation_time: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at))) 
             FROM orders 
             WHERE status = 'ready' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        Ok(OrderMetrics {
            total_orders,
            completed_orders,
            cancelled_orders,
            pending_orders,
            average_order_value: average_order_value.unwrap_or(0.0),
            peak_hours,
            order_completion_rate,
            average_preparation_time: average_preparation_time.unwrap_or(0.0),
        })
    }

    async fn get_revenue_metrics(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
        restaurant_id: Option<Uuid>,
    ) -> Result<RevenueMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        let restaurant_filter = if let Some(id) = restaurant_id {
            format!("AND restaurant_id = '{}'", id)
        } else {
            String::new()
        };

        let total_revenue: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT SUM(total_amount) FROM orders 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let delivery_fees: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT SUM(delivery_fee) FROM orders 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let taxes_collected: Option<f64> = sqlx::query_scalar(&format!(
            "SELECT SUM(tax_amount) FROM orders 
             WHERE status = 'delivered' AND created_at BETWEEN $1 AND $2 {}", restaurant_filter
        ))
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let total_revenue_val = total_revenue.unwrap_or(0.0);
        let delivery_fees_val = delivery_fees.unwrap_or(0.0);
        let taxes_collected_val = taxes_collected.unwrap_or(0.0);
        
        let platform_commission = total_revenue_val * 0.15; // 15% commission
        let restaurant_revenue = total_revenue_val - delivery_fees_val - platform_commission;
        let refunds_issued = 0.0; // TODO: Implement refunds tracking
        let net_revenue = total_revenue_val - refunds_issued;

        let revenue_by_city = self.get_revenue_by_city(start_date, end_date, restaurant_id).await?;

        Ok(RevenueMetrics {
            total_revenue: total_revenue_val,
            restaurant_revenue,
            delivery_fees: delivery_fees_val,
            platform_commission,
            taxes_collected: taxes_collected_val,
            refunds_issued,
            net_revenue,
            revenue_by_city,
        })
    }

    async fn get_customer_metrics(
        &self,
        start_date: &DateTime<Utc>,
        end_date: &DateTime<Utc>,
    ) -> Result<CustomerMetrics> {
        let mut conn = self.database.pool().acquire().await?;

        let total_customers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'customer'")
            .fetch_one(&mut *conn)
            .await?;

        let new_customers: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE role = 'customer' AND created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let returning_customers: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT customer_id) FROM orders 
             WHERE customer_id IN (
                 SELECT customer_id FROM orders 
                 WHERE created_at < $1 
                 GROUP BY customer_id 
                 HAVING COUNT(*) > 0
             ) AND created_at BETWEEN $1 AND $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let customer_retention_rate = if total_customers > 0 {
            returning_customers as f64 / total_customers as f64 * 100.0
        } else {
            0.0
        };

        let average_orders_per_customer: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(order_count) FROM (
                 SELECT COUNT(*) as order_count 
                 FROM orders 
                 WHERE created_at BETWEEN $1 AND $2 
                 GROUP BY customer_id
             ) as customer_orders"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&mut *conn)
        .await?;

        let customer_lifetime_value: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(total_spent) FROM (
                 SELECT SUM(total_amount) as total_spent 
                 FROM orders 
                 WHERE status = 'delivered' 
                 GROUP BY customer_id
             ) as customer_spending"
        )
        .fetch_one(&mut *conn)
        .await?;

        let top_customers = self.get_top_customers(start_date, end_date).await?;

        Ok(CustomerMetrics {
            total_customers,
            new_customers,
            returning_customers,
            customer_retention_rate,
            average_orders_per_customer: average_orders_per_customer.unwrap_or(0.0),
            customer_lifetime_value: customer_lifetime_value.unwrap_or(0.0),
            top_customers,
        })
    }

    async fn get_performance_metrics(&self) -> Result<PerformanceMetrics> {
        // These would typically come from monitoring systems like Prometheus
        // For now, we'll return mock data
        Ok(PerformanceMetrics {
            average_response_time: 150.0, // ms
            system_uptime: 99.9,          // %
            error_rate: 0.1,              // %
            api_calls_per_minute: 1250.0,
            database_performance: DatabasePerformance {
                average_query_time: 25.0, // ms
                slow_queries: 2,
                connection_pool_usage: 65.0, // %
                active_connections: 15,
            },
            cache_hit_rate: 85.0, // %
        })
    }

    // Helper methods
    fn parse_date_range(&self, query: &AnalyticsQuery) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
        if let (Some(start), Some(end)) = (&query.start_date, &query.end_date) {
            return Ok((*start, *end));
        }

        let now = Utc::now();
        let period = query.period.as_deref().unwrap_or("today");

        match period {
            "today" => Ok((now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc(), now)),
            "yesterday" => {
                let yesterday = now - Duration::days(1);
                Ok((yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc(), 
                    yesterday.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc()))
            }
            "this_week" => {
                let start_of_week = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                Ok((start_of_week.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc(), now))
            }
            "last_week" => {
                let end_of_last_week = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                let start_of_last_week = end_of_last_week - Duration::days(7);
                Ok((start_of_last_week.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc(), 
                    end_of_last_week.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc()))
            }
            "this_month" => {
                let start_of_month = now.date_naive().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
                Ok((start_of_month, now))
            }
            "last_month" => {
                let start_of_this_month = now.date_naive().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
                let start_of_last_month = (start_of_this_month - Duration::days(1))
                    .date_naive().with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
                Ok((start_of_last_month, start_of_this_month - Duration::seconds(1)))
            }
            _ => Err(AppError::BadRequest("Invalid period specified".to_string())),
        }
    }

    fn determine_period(&self, query: &AnalyticsQuery) -> AnalyticsPeriod {
        if let (Some(start), Some(end)) = (&query.start_date, &query.end_date) {
            return AnalyticsPeriod::Custom { start: *start, end: *end };
        }

        match query.period.as_deref().unwrap_or("today") {
            "today" => AnalyticsPeriod::Today,
            "yesterday" => AnalyticsPeriod::Yesterday,
            "this_week" => AnalyticsPeriod::ThisWeek,
            "last_week" => AnalyticsPeriod::LastWeek,
            "this_month" => AnalyticsPeriod::ThisMonth,
            "last_month" => AnalyticsPeriod::LastMonth,
            _ => AnalyticsPeriod::Today,
        }
    }

    async fn calculate_system_load(&self) -> Result<f64> {
        // This would typically integrate with system monitoring
        // For now, return a mock value
        Ok(0.65) // 65% system load
    }

    // Additional helper methods would be implemented here...
    async fn get_top_restaurants(&self, _start: &DateTime<Utc>, _end: &DateTime<Utc>, _restaurant_id: Option<Uuid>) -> Result<Vec<RestaurantPerformance>> {
        // Implementation would query top performing restaurants
        Ok(vec![])
    }

    async fn get_top_delivery_performers(&self, _start: &DateTime<Utc>, _end: &DateTime<Utc>) -> Result<Vec<DeliveryPersonPerformance>> {
        // Implementation would query top delivery performers
        Ok(vec![])
    }

    async fn get_peak_hours(&self, _start: &DateTime<Utc>, _end: &DateTime<Utc>, _restaurant_id: Option<Uuid>) -> Result<Vec<PeakHourData>> {
        // Implementation would analyze peak hours
        Ok(vec![])
    }

    async fn get_revenue_by_city(&self, _start: &DateTime<Utc>, _end: &DateTime<Utc>, _restaurant_id: Option<Uuid>) -> Result<Vec<CityRevenue>> {
        // Implementation would group revenue by city
        Ok(vec![])
    }

    async fn get_top_customers(&self, _start: &DateTime<Utc>, _end: &DateTime<Utc>) -> Result<Vec<CustomerPerformance>> {
        // Implementation would query top customers
        Ok(vec![])
    }

    async fn generate_demand_forecast(&self) -> Result<Vec<DemandForecast>> {
        // Implementation would use ML models for demand forecasting
        Ok(vec![])
    }

    async fn generate_revenue_projection(&self) -> Result<RevenueProjection> {
        // Implementation would project revenue based on trends
        Ok(RevenueProjection {
            next_week: 50000.0,
            next_month: 200000.0,
            growth_rate: 15.5,
            seasonal_factors: vec![],
        })
    }

    async fn generate_capacity_recommendations(&self) -> Result<CapacityRecommendations> {
        // Implementation would analyze capacity needs
        Ok(CapacityRecommendations {
            recommended_delivery_persons: 25,
            peak_hour_staffing: vec![],
            restaurant_capacity_alerts: vec![],
        })
    }

    async fn generate_market_insights(&self) -> Result<MarketInsights> {
        // Implementation would analyze market trends
        Ok(MarketInsights {
            trending_cuisines: vec![],
            competitor_analysis: CompetitorAnalysis {
                market_share: 12.5,
                pricing_comparison: 0.95,
                service_quality_comparison: 1.15,
                recommendations: vec![
                    "Consider expanding North Indian cuisine options".to_string(),
                    "Optimize delivery times during peak hours".to_string(),
                ],
            },
            customer_behavior_insights: vec![],
        })
    }
}
