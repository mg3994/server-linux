use prometheus::{
    Gauge, Histogram, IntCounter, IntGauge, Registry, Opts, HistogramOpts,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::delivery::models::{DeliveryStatus, VehicleType};

#[derive(Clone)]
pub struct DeliveryMetrics {
    // Counters
    pub delivery_persons_registered: IntCounter,
    pub delivery_persons_verified: IntCounter,
    pub delivery_persons_deactivated: IntCounter,
    pub orders_assigned: IntCounter,
    pub orders_completed: IntCounter,
    pub orders_cancelled: IntCounter,
    pub orders_failed: IntCounter,
    pub location_updates: IntCounter,
    pub status_updates: IntCounter,
    pub emergency_alerts: IntCounter,

    // Gauges
    pub active_delivery_persons: IntGauge,
    pub available_delivery_persons: IntGauge,
    pub busy_delivery_persons: IntGauge,
    pub pending_assignments: IntGauge,
    pub active_deliveries: IntGauge,
    pub websocket_connections: IntGauge,

    // Histograms
    pub delivery_time: Histogram,
    pub assignment_time: Histogram,
    pub pickup_time: Histogram,
    pub distance_traveled: Histogram,
    pub earnings_per_delivery: Histogram,

    // Vehicle type counters
    pub bicycle_deliveries: IntCounter,
    pub motorcycle_deliveries: IntCounter,
    pub scooter_deliveries: IntCounter,
    pub car_deliveries: IntCounter,
    pub van_deliveries: IntCounter,

    // Status transition counters
    pub status_assigned: IntCounter,
    pub status_accepted: IntCounter,
    pub status_enroute_restaurant: IntCounter,
    pub status_arrived_restaurant: IntCounter,
    pub status_picked_up: IntCounter,
    pub status_enroute_customer: IntCounter,
    pub status_arrived_customer: IntCounter,
    pub status_delivered: IntCounter,

    // Performance metrics
    pub average_rating: Gauge,
    pub success_rate: Gauge,
    pub on_time_delivery_rate: Gauge,

    // Real-time tracking
    active_assignments: Arc<RwLock<HashMap<Uuid, AssignmentMetrics>>>,
}

#[derive(Debug, Clone)]
struct AssignmentMetrics {
    assignment_id: Uuid,
    delivery_person_id: Uuid,
    order_id: Uuid,
    assigned_at: DateTime<Utc>,
    accepted_at: Option<DateTime<Utc>>,
    picked_up_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    current_status: DeliveryStatus,
    vehicle_type: VehicleType,
    estimated_delivery_time: Option<DateTime<Utc>>,
    distance_km: Option<f64>,
}

impl DeliveryMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        // Counters
        let delivery_persons_registered = IntCounter::with_opts(
            Opts::new("delivery_persons_registered_total", "Total number of delivery persons registered")
        )?;
        registry.register(Box::new(delivery_persons_registered.clone()))?;

        let delivery_persons_verified = IntCounter::with_opts(
            Opts::new("delivery_persons_verified_total", "Total number of delivery persons verified")
        )?;
        registry.register(Box::new(delivery_persons_verified.clone()))?;

        let delivery_persons_deactivated = IntCounter::with_opts(
            Opts::new("delivery_persons_deactivated_total", "Total number of delivery persons deactivated")
        )?;
        registry.register(Box::new(delivery_persons_deactivated.clone()))?;

        let orders_assigned = IntCounter::with_opts(
            Opts::new("delivery_orders_assigned_total", "Total number of orders assigned to delivery persons")
        )?;
        registry.register(Box::new(orders_assigned.clone()))?;

        let orders_completed = IntCounter::with_opts(
            Opts::new("delivery_orders_completed_total", "Total number of orders completed")
        )?;
        registry.register(Box::new(orders_completed.clone()))?;

        let orders_cancelled = IntCounter::with_opts(
            Opts::new("delivery_orders_cancelled_total", "Total number of orders cancelled")
        )?;
        registry.register(Box::new(orders_cancelled.clone()))?;

        let orders_failed = IntCounter::with_opts(
            Opts::new("delivery_orders_failed_total", "Total number of orders failed")
        )?;
        registry.register(Box::new(orders_failed.clone()))?;

        let location_updates = IntCounter::with_opts(
            Opts::new("delivery_location_updates_total", "Total number of location updates")
        )?;
        registry.register(Box::new(location_updates.clone()))?;

        let status_updates = IntCounter::with_opts(
            Opts::new("delivery_status_updates_total", "Total number of status updates")
        )?;
        registry.register(Box::new(status_updates.clone()))?;

        let emergency_alerts = IntCounter::with_opts(
            Opts::new("delivery_emergency_alerts_total", "Total number of emergency alerts")
        )?;
        registry.register(Box::new(emergency_alerts.clone()))?;

        // Gauges
        let active_delivery_persons = IntGauge::with_opts(
            Opts::new("delivery_persons_active", "Number of active delivery persons")
        )?;
        registry.register(Box::new(active_delivery_persons.clone()))?;

        let available_delivery_persons = IntGauge::with_opts(
            Opts::new("delivery_persons_available", "Number of available delivery persons")
        )?;
        registry.register(Box::new(available_delivery_persons.clone()))?;

        let busy_delivery_persons = IntGauge::with_opts(
            Opts::new("delivery_persons_busy", "Number of busy delivery persons")
        )?;
        registry.register(Box::new(busy_delivery_persons.clone()))?;

        let pending_assignments = IntGauge::with_opts(
            Opts::new("delivery_assignments_pending", "Number of pending delivery assignments")
        )?;
        registry.register(Box::new(pending_assignments.clone()))?;

        let active_deliveries = IntGauge::with_opts(
            Opts::new("delivery_active_deliveries", "Number of active deliveries")
        )?;
        registry.register(Box::new(active_deliveries.clone()))?;

        let websocket_connections = IntGauge::with_opts(
            Opts::new("delivery_websocket_connections", "Number of active WebSocket connections")
        )?;
        registry.register(Box::new(websocket_connections.clone()))?;

        // Histograms
        let delivery_time = Histogram::with_opts(
            HistogramOpts::new("delivery_time_minutes", "Time taken for delivery in minutes")
                .buckets(vec![10.0, 20.0, 30.0, 45.0, 60.0, 90.0, 120.0])
        )?;
        registry.register(Box::new(delivery_time.clone()))?;

        let assignment_time = Histogram::with_opts(
            HistogramOpts::new("delivery_assignment_time_seconds", "Time taken to assign order in seconds")
                .buckets(vec![1.0, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0])
        )?;
        registry.register(Box::new(assignment_time.clone()))?;

        let pickup_time = Histogram::with_opts(
            HistogramOpts::new("delivery_pickup_time_minutes", "Time taken to pickup order in minutes")
                .buckets(vec![5.0, 10.0, 15.0, 20.0, 30.0, 45.0, 60.0])
        )?;
        registry.register(Box::new(pickup_time.clone()))?;

        let distance_traveled = Histogram::with_opts(
            HistogramOpts::new("delivery_distance_km", "Distance traveled for delivery in kilometers")
                .buckets(vec![1.0, 2.0, 5.0, 10.0, 15.0, 20.0, 30.0])
        )?;
        registry.register(Box::new(distance_traveled.clone()))?;

        let earnings_per_delivery = Histogram::with_opts(
            HistogramOpts::new("delivery_earnings_rupees", "Earnings per delivery in rupees")
                .buckets(vec![20.0, 30.0, 50.0, 75.0, 100.0, 150.0, 200.0])
        )?;
        registry.register(Box::new(earnings_per_delivery.clone()))?;

        // Vehicle type counters
        let bicycle_deliveries = IntCounter::with_opts(
            Opts::new("delivery_bicycle_total", "Total deliveries by bicycle")
        )?;
        registry.register(Box::new(bicycle_deliveries.clone()))?;

        let motorcycle_deliveries = IntCounter::with_opts(
            Opts::new("delivery_motorcycle_total", "Total deliveries by motorcycle")
        )?;
        registry.register(Box::new(motorcycle_deliveries.clone()))?;

        let scooter_deliveries = IntCounter::with_opts(
            Opts::new("delivery_scooter_total", "Total deliveries by scooter")
        )?;
        registry.register(Box::new(scooter_deliveries.clone()))?;

        let car_deliveries = IntCounter::with_opts(
            Opts::new("delivery_car_total", "Total deliveries by car")
        )?;
        registry.register(Box::new(car_deliveries.clone()))?;

        let van_deliveries = IntCounter::with_opts(
            Opts::new("delivery_van_total", "Total deliveries by van")
        )?;
        registry.register(Box::new(van_deliveries.clone()))?;

        // Status transition counters
        let status_assigned = IntCounter::with_opts(
            Opts::new("delivery_status_assigned_total", "Total orders assigned")
        )?;
        registry.register(Box::new(status_assigned.clone()))?;

        let status_accepted = IntCounter::with_opts(
            Opts::new("delivery_status_accepted_total", "Total orders accepted")
        )?;
        registry.register(Box::new(status_accepted.clone()))?;

        let status_enroute_restaurant = IntCounter::with_opts(
            Opts::new("delivery_status_enroute_restaurant_total", "Total orders en route to restaurant")
        )?;
        registry.register(Box::new(status_enroute_restaurant.clone()))?;

        let status_arrived_restaurant = IntCounter::with_opts(
            Opts::new("delivery_status_arrived_restaurant_total", "Total orders arrived at restaurant")
        )?;
        registry.register(Box::new(status_arrived_restaurant.clone()))?;

        let status_picked_up = IntCounter::with_opts(
            Opts::new("delivery_status_picked_up_total", "Total orders picked up")
        )?;
        registry.register(Box::new(status_picked_up.clone()))?;

        let status_enroute_customer = IntCounter::with_opts(
            Opts::new("delivery_status_enroute_customer_total", "Total orders en route to customer")
        )?;
        registry.register(Box::new(status_enroute_customer.clone()))?;

        let status_arrived_customer = IntCounter::with_opts(
            Opts::new("delivery_status_arrived_customer_total", "Total orders arrived at customer")
        )?;
        registry.register(Box::new(status_arrived_customer.clone()))?;

        let status_delivered = IntCounter::with_opts(
            Opts::new("delivery_status_delivered_total", "Total orders delivered")
        )?;
        registry.register(Box::new(status_delivered.clone()))?;

        // Performance metrics
        let average_rating = Gauge::with_opts(
            Opts::new("delivery_average_rating", "Average rating of delivery persons")
        )?;
        registry.register(Box::new(average_rating.clone()))?;

        let success_rate = Gauge::with_opts(
            Opts::new("delivery_success_rate", "Success rate of deliveries (0-1)")
        )?;
        registry.register(Box::new(success_rate.clone()))?;

        let on_time_delivery_rate = Gauge::with_opts(
            Opts::new("delivery_on_time_rate", "On-time delivery rate (0-1)")
        )?;
        registry.register(Box::new(on_time_delivery_rate.clone()))?;

        Ok(Self {
            delivery_persons_registered,
            delivery_persons_verified,
            delivery_persons_deactivated,
            orders_assigned,
            orders_completed,
            orders_cancelled,
            orders_failed,
            location_updates,
            status_updates,
            emergency_alerts,
            active_delivery_persons,
            available_delivery_persons,
            busy_delivery_persons,
            pending_assignments,
            active_deliveries,
            websocket_connections,
            delivery_time,
            assignment_time,
            pickup_time,
            distance_traveled,
            earnings_per_delivery,
            bicycle_deliveries,
            motorcycle_deliveries,
            scooter_deliveries,
            car_deliveries,
            van_deliveries,
            status_assigned,
            status_accepted,
            status_enroute_restaurant,
            status_arrived_restaurant,
            status_picked_up,
            status_enroute_customer,
            status_arrived_customer,
            status_delivered,
            average_rating,
            success_rate,
            on_time_delivery_rate,
            active_assignments: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // Increment methods
    pub fn increment_delivery_person_registered(&self) {
        self.delivery_persons_registered.inc();
    }

    pub fn increment_delivery_person_verified(&self) {
        self.delivery_persons_verified.inc();
    }

    pub fn increment_delivery_person_deactivated(&self) {
        self.delivery_persons_deactivated.inc();
    }

    pub fn increment_order_assigned(&self) {
        self.orders_assigned.inc();
        self.status_assigned.inc();
    }

    pub fn increment_order_completed(&self, vehicle_type: VehicleType) {
        self.orders_completed.inc();
        self.status_delivered.inc();
        
        match vehicle_type {
            VehicleType::Bicycle => self.bicycle_deliveries.inc(),
            VehicleType::Motorcycle => self.motorcycle_deliveries.inc(),
            VehicleType::Scooter => self.scooter_deliveries.inc(),
            VehicleType::Car => self.car_deliveries.inc(),
            VehicleType::Van => self.van_deliveries.inc(),
        }
    }

    pub fn increment_order_cancelled(&self) {
        self.orders_cancelled.inc();
    }

    pub fn increment_order_failed(&self) {
        self.orders_failed.inc();
    }

    pub fn increment_location_update(&self) {
        self.location_updates.inc();
    }

    pub fn increment_status_update(&self, status: &DeliveryStatus) {
        self.status_updates.inc();
        
        match status {
            DeliveryStatus::Assigned => self.status_assigned.inc(),
            DeliveryStatus::Accepted => self.status_accepted.inc(),
            DeliveryStatus::EnRouteToRestaurant => self.status_enroute_restaurant.inc(),
            DeliveryStatus::ArrivedAtRestaurant => self.status_arrived_restaurant.inc(),
            DeliveryStatus::PickedUp => self.status_picked_up.inc(),
            DeliveryStatus::EnRouteToCustomer => self.status_enroute_customer.inc(),
            DeliveryStatus::ArrivedAtCustomer => self.status_arrived_customer.inc(),
            DeliveryStatus::Delivered => self.status_delivered.inc(),
            _ => {}
        }
    }

    pub fn increment_emergency_alert(&self) {
        self.emergency_alerts.inc();
    }

    // Gauge update methods
    pub fn set_active_delivery_persons(&self, count: i64) {
        self.active_delivery_persons.set(count);
    }

    pub fn set_available_delivery_persons(&self, count: i64) {
        self.available_delivery_persons.set(count);
    }

    pub fn set_busy_delivery_persons(&self, count: i64) {
        self.busy_delivery_persons.set(count);
    }

    pub fn set_pending_assignments(&self, count: i64) {
        self.pending_assignments.set(count);
    }

    pub fn set_active_deliveries(&self, count: i64) {
        self.active_deliveries.set(count);
    }

    pub fn set_websocket_connections(&self, count: i64) {
        self.websocket_connections.set(count);
    }

    // Histogram observation methods
    pub fn observe_delivery_time(&self, minutes: f64) {
        self.delivery_time.observe(minutes);
    }

    pub fn observe_assignment_time(&self, seconds: f64) {
        self.assignment_time.observe(seconds);
    }

    pub fn observe_pickup_time(&self, minutes: f64) {
        self.pickup_time.observe(minutes);
    }

    pub fn observe_distance_traveled(&self, km: f64) {
        self.distance_traveled.observe(km);
    }

    pub fn observe_earnings(&self, rupees: f64) {
        self.earnings_per_delivery.observe(rupees);
    }

    // Performance metrics
    pub fn update_average_rating(&self, rating: f64) {
        self.average_rating.set(rating);
    }

    pub fn update_success_rate(&self, rate: f64) {
        self.success_rate.set(rate);
    }

    pub fn update_on_time_delivery_rate(&self, rate: f64) {
        self.on_time_delivery_rate.set(rate);
    }

    // Assignment tracking
    pub async fn start_assignment_tracking(
        &self,
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        order_id: Uuid,
        vehicle_type: VehicleType,
        estimated_delivery_time: Option<DateTime<Utc>>,
    ) {
        let metrics = AssignmentMetrics {
            assignment_id,
            delivery_person_id,
            order_id,
            assigned_at: Utc::now(),
            accepted_at: None,
            picked_up_at: None,
            delivered_at: None,
            current_status: DeliveryStatus::Assigned,
            vehicle_type,
            estimated_delivery_time,
            distance_km: None,
        };

        let mut assignments = self.active_assignments.write().await;
        assignments.insert(assignment_id, metrics);
    }

    pub async fn update_assignment_status(
        &self,
        assignment_id: Uuid,
        status: DeliveryStatus,
    ) {
        let mut assignments = self.active_assignments.write().await;
        if let Some(assignment) = assignments.get_mut(&assignment_id) {
            let now = Utc::now();
            
            match status {
                DeliveryStatus::Accepted => assignment.accepted_at = Some(now),
                DeliveryStatus::PickedUp => assignment.picked_up_at = Some(now),
                DeliveryStatus::Delivered => {
                    assignment.delivered_at = Some(now);
                    
                    // Calculate and observe delivery time
                    let delivery_time_minutes = (now - assignment.assigned_at).num_minutes() as f64;
                    self.observe_delivery_time(delivery_time_minutes);
                    
                    // Calculate and observe pickup time if available
                    if let Some(picked_up_at) = assignment.picked_up_at {
                        let pickup_time_minutes = (picked_up_at - assignment.assigned_at).num_minutes() as f64;
                        self.observe_pickup_time(pickup_time_minutes);
                    }
                    
                    // Observe distance if available
                    if let Some(distance) = assignment.distance_km {
                        self.observe_distance_traveled(distance);
                    }
                }
                _ => {}
            }
            
            assignment.current_status = status;
        }
    }

    pub async fn complete_assignment(&self, assignment_id: Uuid, earnings: f64) {
        let mut assignments = self.active_assignments.write().await;
        if let Some(assignment) = assignments.remove(&assignment_id) {
            self.observe_earnings(earnings);
            self.increment_order_completed(assignment.vehicle_type);
        }
    }

    pub async fn get_active_assignment_count(&self) -> usize {
        let assignments = self.active_assignments.read().await;
        assignments.len()
    }
}