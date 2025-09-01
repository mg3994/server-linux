use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Restaurant {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cuisine_type: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: String,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub image_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub rating: f64,
    pub total_reviews: i32,
    pub delivery_fee: f64,
    pub minimum_order: f64,
    pub delivery_time_minutes: i32,
    pub is_active: bool,
    pub is_accepting_orders: bool,
    pub fssai_license: Option<String>,
    pub gst_number: Option<String>,
    pub opening_hours: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MenuItem {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub is_vegetarian: bool,
    pub is_vegan: bool,
    pub is_gluten_free: bool,
    pub spice_level: i32, // 0-5 scale
    pub ingredients: Option<Vec<String>>,
    pub allergens: Option<Vec<String>>,
    pub is_available: bool,
    pub preparation_time_minutes: i32,
    pub calories: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRestaurantRequest {
    pub name: String,
    pub description: Option<String>,
    pub cuisine_type: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub phone: String,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub image_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub delivery_fee: f64,
    pub minimum_order: f64,
    pub delivery_time_minutes: i32,
    pub fssai_license: Option<String>,
    pub gst_number: Option<String>,
    pub opening_hours: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRestaurantRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cuisine_type: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub image_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub delivery_fee: Option<f64>,
    pub minimum_order: Option<f64>,
    pub delivery_time_minutes: Option<i32>,
    pub is_accepting_orders: Option<bool>,
    pub fssai_license: Option<String>,
    pub gst_number: Option<String>,
    pub opening_hours: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub is_vegetarian: bool,
    pub is_vegan: bool,
    pub is_gluten_free: bool,
    pub spice_level: i32,
    pub ingredients: Option<Vec<String>>,
    pub allergens: Option<Vec<String>>,
    pub preparation_time_minutes: i32,
    pub calories: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<String>,
    pub is_vegetarian: Option<bool>,
    pub is_vegan: Option<bool>,
    pub is_gluten_free: Option<bool>,
    pub spice_level: Option<i32>,
    pub ingredients: Option<Vec<String>>,
    pub allergens: Option<Vec<String>>,
    pub is_available: Option<bool>,
    pub preparation_time_minutes: Option<i32>,
    pub calories: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RestaurantResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cuisine_type: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: String,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub image_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub rating: f64,
    pub total_reviews: i32,
    pub delivery_fee: f64,
    pub minimum_order: f64,
    pub delivery_time_minutes: i32,
    pub is_active: bool,
    pub is_accepting_orders: bool,
    pub opening_hours: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Restaurant> for RestaurantResponse {
    fn from(restaurant: Restaurant) -> Self {
        Self {
            id: restaurant.id,
            name: restaurant.name,
            description: restaurant.description,
            cuisine_type: restaurant.cuisine_type,
            address: restaurant.address,
            city: restaurant.city,
            state: restaurant.state,
            postal_code: restaurant.postal_code,
            country: restaurant.country,
            phone: restaurant.phone,
            email: restaurant.email,
            latitude: restaurant.latitude,
            longitude: restaurant.longitude,
            image_url: restaurant.image_url,
            cover_image_url: restaurant.cover_image_url,
            rating: restaurant.rating,
            total_reviews: restaurant.total_reviews,
            delivery_fee: restaurant.delivery_fee,
            minimum_order: restaurant.minimum_order,
            delivery_time_minutes: restaurant.delivery_time_minutes,
            is_active: restaurant.is_active,
            is_accepting_orders: restaurant.is_accepting_orders,
            opening_hours: restaurant.opening_hours,
            created_at: restaurant.created_at,
            updated_at: restaurant.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MenuItemResponse {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub is_vegetarian: bool,
    pub is_vegan: bool,
    pub is_gluten_free: bool,
    pub spice_level: i32,
    pub ingredients: Option<Vec<String>>,
    pub allergens: Option<Vec<String>>,
    pub is_available: bool,
    pub preparation_time_minutes: i32,
    pub calories: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<MenuItem> for MenuItemResponse {
    fn from(item: MenuItem) -> Self {
        Self {
            id: item.id,
            restaurant_id: item.restaurant_id,
            name: item.name,
            description: item.description,
            category: item.category,
            price: item.price,
            image_url: item.image_url,
            is_vegetarian: item.is_vegetarian,
            is_vegan: item.is_vegan,
            is_gluten_free: item.is_gluten_free,
            spice_level: item.spice_level,
            ingredients: item.ingredients,
            allergens: item.allergens,
            is_available: item.is_available,
            preparation_time_minutes: item.preparation_time_minutes,
            calories: item.calories,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RestaurantListResponse {
    pub restaurants: Vec<RestaurantResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Serialize)]
pub struct MenuResponse {
    pub restaurant_id: Uuid,
    pub categories: Vec<MenuCategory>,
}

#[derive(Debug, Serialize)]
pub struct MenuCategory {
    pub name: String,
    pub items: Vec<MenuItemResponse>,
}
