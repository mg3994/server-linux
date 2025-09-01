use crate::database::Database;
use crate::error::{AppError, Result};
use crate::restaurants::models::*;
use uuid::Uuid;
use chrono::Utc;
// Using f64 for decimal values for better PostgreSQL compatibility
use std::collections::HashMap;
use sqlx::Row;

pub struct RestaurantService {
    db: Database,
}

impl RestaurantService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_restaurant(&self, owner_id: Uuid, request: CreateRestaurantRequest) -> Result<Restaurant> {
        let restaurant_id = Uuid::new_v4();
        let now = Utc::now();

        let restaurant = sqlx::query_as::<_, Restaurant>(
            r#"
            INSERT INTO restaurants (
                id, owner_id, name, description, cuisine_type, address, city, state, 
                postal_code, country, phone, email, latitude, longitude, image_url, 
                cover_image_url, rating, total_reviews, delivery_fee, minimum_order, 
                delivery_time_minutes, is_active, is_accepting_orders, fssai_license, 
                gst_number, opening_hours, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, 
                $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28
            ) RETURNING *
            "#,
        )
        .bind(restaurant_id)
        .bind(owner_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.cuisine_type)
        .bind(&request.address)
        .bind(&request.city)
        .bind(&request.state)
        .bind(&request.postal_code)
        .bind("India") // Default country for India-focused platform
        .bind(&request.phone)
        .bind(&request.email)
        .bind(&request.latitude)
        .bind(&request.longitude)
        .bind(&request.image_url)
        .bind(&request.cover_image_url)
        .bind(0.0f64) // Initial rating
        .bind(0i32) // Initial review count
        .bind(&request.delivery_fee)
        .bind(&request.minimum_order)
        .bind(request.delivery_time_minutes)
        .bind(true) // is_active
        .bind(true) // is_accepting_orders
        .bind(&request.fssai_license)
        .bind(&request.gst_number)
        .bind(&request.opening_hours)
        .bind(now)
        .bind(now)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(restaurant)
    }

    pub async fn get_restaurant(&self, restaurant_id: Uuid) -> Result<Restaurant> {
        let restaurant = sqlx::query_as::<_, Restaurant>(
            "SELECT * FROM restaurants WHERE id = $1 AND is_active = true"
        )
        .bind(restaurant_id)
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Restaurant not found".to_string()))?;

        Ok(restaurant)
    }

    pub async fn update_restaurant(&self, restaurant_id: Uuid, owner_id: Uuid, request: UpdateRestaurantRequest) -> Result<Restaurant> {
        let now = Utc::now();

        // Build dynamic update query
        let mut query_parts = Vec::new();
        let mut bind_count = 1;

        if request.name.is_some() {
            query_parts.push(format!("name = ${}", bind_count));
            bind_count += 1;
        }
        if request.description.is_some() {
            query_parts.push(format!("description = ${}", bind_count));
            bind_count += 1;
        }
        if request.cuisine_type.is_some() {
            query_parts.push(format!("cuisine_type = ${}", bind_count));
            bind_count += 1;
        }
        if request.address.is_some() {
            query_parts.push(format!("address = ${}", bind_count));
            bind_count += 1;
        }
        if request.city.is_some() {
            query_parts.push(format!("city = ${}", bind_count));
            bind_count += 1;
        }
        if request.state.is_some() {
            query_parts.push(format!("state = ${}", bind_count));
            bind_count += 1;
        }
        if request.postal_code.is_some() {
            query_parts.push(format!("postal_code = ${}", bind_count));
            bind_count += 1;
        }
        if request.phone.is_some() {
            query_parts.push(format!("phone = ${}", bind_count));
            bind_count += 1;
        }
        if request.email.is_some() {
            query_parts.push(format!("email = ${}", bind_count));
            bind_count += 1;
        }
        if request.latitude.is_some() {
            query_parts.push(format!("latitude = ${}", bind_count));
            bind_count += 1;
        }
        if request.longitude.is_some() {
            query_parts.push(format!("longitude = ${}", bind_count));
            bind_count += 1;
        }
        if request.image_url.is_some() {
            query_parts.push(format!("image_url = ${}", bind_count));
            bind_count += 1;
        }
        if request.cover_image_url.is_some() {
            query_parts.push(format!("cover_image_url = ${}", bind_count));
            bind_count += 1;
        }
        if request.delivery_fee.is_some() {
            query_parts.push(format!("delivery_fee = ${}", bind_count));
            bind_count += 1;
        }
        if request.minimum_order.is_some() {
            query_parts.push(format!("minimum_order = ${}", bind_count));
            bind_count += 1;
        }
        if request.delivery_time_minutes.is_some() {
            query_parts.push(format!("delivery_time_minutes = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_accepting_orders.is_some() {
            query_parts.push(format!("is_accepting_orders = ${}", bind_count));
            bind_count += 1;
        }
        if request.fssai_license.is_some() {
            query_parts.push(format!("fssai_license = ${}", bind_count));
            bind_count += 1;
        }
        if request.gst_number.is_some() {
            query_parts.push(format!("gst_number = ${}", bind_count));
            bind_count += 1;
        }
        if request.opening_hours.is_some() {
            query_parts.push(format!("opening_hours = ${}", bind_count));
            bind_count += 1;
        }

        if query_parts.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        query_parts.push(format!("updated_at = ${}", bind_count));
        let update_clause = query_parts.join(", ");

        let query = format!(
            "UPDATE restaurants SET {} WHERE id = ${} AND owner_id = ${} AND is_active = true RETURNING *",
            update_clause,
            bind_count + 1,
            bind_count + 2
        );

        let mut query_builder = sqlx::query_as::<_, Restaurant>(&query);

        // Bind parameters in the same order as the query parts
        if let Some(name) = &request.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(description) = &request.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(cuisine_type) = &request.cuisine_type {
            query_builder = query_builder.bind(cuisine_type);
        }
        if let Some(address) = &request.address {
            query_builder = query_builder.bind(address);
        }
        if let Some(city) = &request.city {
            query_builder = query_builder.bind(city);
        }
        if let Some(state) = &request.state {
            query_builder = query_builder.bind(state);
        }
        if let Some(postal_code) = &request.postal_code {
            query_builder = query_builder.bind(postal_code);
        }
        if let Some(phone) = &request.phone {
            query_builder = query_builder.bind(phone);
        }
        if let Some(email) = &request.email {
            query_builder = query_builder.bind(email);
        }
        if let Some(latitude) = &request.latitude {
            query_builder = query_builder.bind(latitude);
        }
        if let Some(longitude) = &request.longitude {
            query_builder = query_builder.bind(longitude);
        }
        if let Some(image_url) = &request.image_url {
            query_builder = query_builder.bind(image_url);
        }
        if let Some(cover_image_url) = &request.cover_image_url {
            query_builder = query_builder.bind(cover_image_url);
        }
        if let Some(delivery_fee) = &request.delivery_fee {
            query_builder = query_builder.bind(delivery_fee);
        }
        if let Some(minimum_order) = &request.minimum_order {
            query_builder = query_builder.bind(minimum_order);
        }
        if let Some(delivery_time_minutes) = &request.delivery_time_minutes {
            query_builder = query_builder.bind(delivery_time_minutes);
        }
        if let Some(is_accepting_orders) = &request.is_accepting_orders {
            query_builder = query_builder.bind(is_accepting_orders);
        }
        if let Some(fssai_license) = &request.fssai_license {
            query_builder = query_builder.bind(fssai_license);
        }
        if let Some(gst_number) = &request.gst_number {
            query_builder = query_builder.bind(gst_number);
        }
        if let Some(opening_hours) = &request.opening_hours {
            query_builder = query_builder.bind(opening_hours);
        }

        let restaurant = query_builder
            .bind(now)
            .bind(restaurant_id)
            .bind(owner_id)
            .fetch_optional(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Restaurant not found or not owned by user".to_string()))?;

        Ok(restaurant)
    }

    pub async fn delete_restaurant(&self, restaurant_id: Uuid, owner_id: Uuid) -> Result<()> {
        let result = sqlx::query(
            "UPDATE restaurants SET is_active = false, updated_at = $1 WHERE id = $2 AND owner_id = $3 AND is_active = true"
        )
        .bind(Utc::now())
        .bind(restaurant_id)
        .bind(owner_id)
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Restaurant not found or not owned by user".to_string()));
        }

        Ok(())
    }

    pub async fn list_restaurants(&self, city: Option<String>, cuisine_type: Option<String>, page: i32, per_page: i32) -> Result<RestaurantListResponse> {
        let offset = (page - 1) * per_page;
        
        let mut where_conditions = vec!["is_active = true", "is_accepting_orders = true"];
        let mut bind_count = 1;
        let city_condition;
        let cuisine_condition;

        if city.is_some() {
            city_condition = format!("LOWER(city) = LOWER(${})", bind_count);
            where_conditions.push(&city_condition);
            bind_count += 1;
        }
        if cuisine_type.is_some() {
            cuisine_condition = format!("LOWER(cuisine_type) = LOWER(${})", bind_count);
            where_conditions.push(&cuisine_condition);
            bind_count += 1;
        }

        let where_clause = where_conditions.join(" AND ");
        
        let query = format!(
            "SELECT * FROM restaurants WHERE {} ORDER BY rating DESC, total_reviews DESC LIMIT ${} OFFSET ${}",
            where_clause,
            bind_count,
            bind_count + 1
        );

        let count_query = format!(
            "SELECT COUNT(*) FROM restaurants WHERE {}",
            where_clause
        );

        let mut query_builder = sqlx::query_as::<_, Restaurant>(&query);
        let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);

        if let Some(ref city_val) = city {
            query_builder = query_builder.bind(city_val);
            count_query_builder = count_query_builder.bind(city_val);
        }
        if let Some(ref cuisine_val) = cuisine_type {
            query_builder = query_builder.bind(cuisine_val);
            count_query_builder = count_query_builder.bind(cuisine_val);
        }

        let restaurants = query_builder
            .bind(per_page)
            .bind(offset)
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let total = count_query_builder
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let restaurant_responses: Vec<RestaurantResponse> = restaurants
            .into_iter()
            .map(RestaurantResponse::from)
            .collect();

        Ok(RestaurantListResponse {
            restaurants: restaurant_responses,
            total,
            page,
            per_page,
        })
    }

    pub async fn search_restaurants(&self, query: String, city: Option<String>, page: i32, per_page: i32) -> Result<RestaurantListResponse> {
        let offset = (page - 1) * per_page;
        let search_term = format!("%{}%", query.to_lowercase());
        
        let mut where_conditions = vec![
            "is_active = true",
            "is_accepting_orders = true",
            "(LOWER(name) LIKE $1 OR LOWER(description) LIKE $1 OR LOWER(cuisine_type) LIKE $1)"
        ];
        let mut bind_count = 2;
        let city_condition;

        if city.is_some() {
            city_condition = format!("LOWER(city) = LOWER(${})", bind_count);
            where_conditions.push(&city_condition);
            bind_count += 1;
        }

        let where_clause = where_conditions.join(" AND ");
        
        let sql_query = format!(
            "SELECT * FROM restaurants WHERE {} ORDER BY rating DESC, total_reviews DESC LIMIT ${} OFFSET ${}",
            where_clause,
            bind_count,
            bind_count + 1
        );

        let count_query = format!(
            "SELECT COUNT(*) FROM restaurants WHERE {}",
            where_clause
        );

        let mut query_builder = sqlx::query_as::<_, Restaurant>(&sql_query);
        let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);

        query_builder = query_builder.bind(&search_term);
        count_query_builder = count_query_builder.bind(&search_term);

        if let Some(ref city_val) = city {
            query_builder = query_builder.bind(city_val);
            count_query_builder = count_query_builder.bind(city_val);
        }

        let restaurants = query_builder
            .bind(per_page)
            .bind(offset)
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let total = count_query_builder
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let restaurant_responses: Vec<RestaurantResponse> = restaurants
            .into_iter()
            .map(RestaurantResponse::from)
            .collect();

        Ok(RestaurantListResponse {
            restaurants: restaurant_responses,
            total,
            page,
            per_page,
        })
    }

    // Menu Management Methods
    pub async fn create_menu_item(&self, restaurant_id: Uuid, owner_id: Uuid, request: CreateMenuItemRequest) -> Result<MenuItem> {
        // First verify the restaurant belongs to the owner
        self.verify_restaurant_ownership(restaurant_id, owner_id).await?;

        let item_id = Uuid::new_v4();
        let now = Utc::now();

        let menu_item = sqlx::query_as::<_, MenuItem>(
            r#"
            INSERT INTO menu_items (
                id, restaurant_id, name, description, category, price, image_url,
                is_vegetarian, is_vegan, is_gluten_free, spice_level, ingredients,
                allergens, is_available, preparation_time_minutes, calories, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            ) RETURNING *
            "#,
        )
        .bind(item_id)
        .bind(restaurant_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.category)
        .bind(&request.price)
        .bind(&request.image_url)
        .bind(request.is_vegetarian)
        .bind(request.is_vegan)
        .bind(request.is_gluten_free)
        .bind(request.spice_level)
        .bind(serde_json::to_value(&request.ingredients).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&request.allergens).unwrap_or(serde_json::Value::Null))
        .bind(true) // is_available by default
        .bind(request.preparation_time_minutes)
        .bind(&request.calories)
        .bind(now)
        .bind(now)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(menu_item)
    }

    pub async fn get_menu(&self, restaurant_id: Uuid) -> Result<MenuResponse> {
        let menu_items = sqlx::query_as::<_, MenuItem>(
            "SELECT * FROM menu_items WHERE restaurant_id = $1 ORDER BY category, name"
        )
        .bind(restaurant_id)
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Group items by category
        let mut categories_map: HashMap<String, Vec<MenuItemResponse>> = HashMap::new();
        
        for item in menu_items {
            let category = item.category.clone();
            let item_response = MenuItemResponse::from(item);
            categories_map.entry(category).or_insert_with(Vec::new).push(item_response);
        }

        let categories: Vec<MenuCategory> = categories_map
            .into_iter()
            .map(|(name, items)| MenuCategory { name, items })
            .collect();

        Ok(MenuResponse {
            restaurant_id,
            categories,
        })
    }

    pub async fn update_menu_item(&self, restaurant_id: Uuid, item_id: Uuid, owner_id: Uuid, request: UpdateMenuItemRequest) -> Result<MenuItem> {
        // First verify the restaurant belongs to the owner
        self.verify_restaurant_ownership(restaurant_id, owner_id).await?;

        let now = Utc::now();

        // Build dynamic update query (similar to restaurant update)
        let mut query_parts = Vec::new();
        let mut bind_count = 1;

        if request.name.is_some() {
            query_parts.push(format!("name = ${}", bind_count));
            bind_count += 1;
        }
        if request.description.is_some() {
            query_parts.push(format!("description = ${}", bind_count));
            bind_count += 1;
        }
        if request.category.is_some() {
            query_parts.push(format!("category = ${}", bind_count));
            bind_count += 1;
        }
        if request.price.is_some() {
            query_parts.push(format!("price = ${}", bind_count));
            bind_count += 1;
        }
        if request.image_url.is_some() {
            query_parts.push(format!("image_url = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_vegetarian.is_some() {
            query_parts.push(format!("is_vegetarian = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_vegan.is_some() {
            query_parts.push(format!("is_vegan = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_gluten_free.is_some() {
            query_parts.push(format!("is_gluten_free = ${}", bind_count));
            bind_count += 1;
        }
        if request.spice_level.is_some() {
            query_parts.push(format!("spice_level = ${}", bind_count));
            bind_count += 1;
        }
        if request.ingredients.is_some() {
            query_parts.push(format!("ingredients = ${}", bind_count));
            bind_count += 1;
        }
        if request.allergens.is_some() {
            query_parts.push(format!("allergens = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_available.is_some() {
            query_parts.push(format!("is_available = ${}", bind_count));
            bind_count += 1;
        }
        if request.preparation_time_minutes.is_some() {
            query_parts.push(format!("preparation_time_minutes = ${}", bind_count));
            bind_count += 1;
        }
        if request.calories.is_some() {
            query_parts.push(format!("calories = ${}", bind_count));
            bind_count += 1;
        }

        if query_parts.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        query_parts.push(format!("updated_at = ${}", bind_count));
        let update_clause = query_parts.join(", ");

        let query = format!(
            "UPDATE menu_items SET {} WHERE id = ${} AND restaurant_id = ${} RETURNING *",
            update_clause,
            bind_count + 1,
            bind_count + 2
        );

        let mut query_builder = sqlx::query_as::<_, MenuItem>(&query);

        // Bind parameters in the same order
        if let Some(name) = &request.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(description) = &request.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(category) = &request.category {
            query_builder = query_builder.bind(category);
        }
        if let Some(price) = &request.price {
            query_builder = query_builder.bind(price);
        }
        if let Some(image_url) = &request.image_url {
            query_builder = query_builder.bind(image_url);
        }
        if let Some(is_vegetarian) = &request.is_vegetarian {
            query_builder = query_builder.bind(is_vegetarian);
        }
        if let Some(is_vegan) = &request.is_vegan {
            query_builder = query_builder.bind(is_vegan);
        }
        if let Some(is_gluten_free) = &request.is_gluten_free {
            query_builder = query_builder.bind(is_gluten_free);
        }
        if let Some(spice_level) = &request.spice_level {
            query_builder = query_builder.bind(spice_level);
        }
        if let Some(ingredients) = &request.ingredients {
            query_builder = query_builder.bind(serde_json::to_value(ingredients).unwrap_or(serde_json::Value::Null));
        }
        if let Some(allergens) = &request.allergens {
            query_builder = query_builder.bind(serde_json::to_value(allergens).unwrap_or(serde_json::Value::Null));
        }
        if let Some(is_available) = &request.is_available {
            query_builder = query_builder.bind(is_available);
        }
        if let Some(preparation_time_minutes) = &request.preparation_time_minutes {
            query_builder = query_builder.bind(preparation_time_minutes);
        }
        if let Some(calories) = &request.calories {
            query_builder = query_builder.bind(calories);
        }

        let menu_item = query_builder
            .bind(now)
            .bind(item_id)
            .bind(restaurant_id)
            .fetch_optional(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Menu item not found".to_string()))?;

        Ok(menu_item)
    }

    pub async fn delete_menu_item(&self, restaurant_id: Uuid, item_id: Uuid, owner_id: Uuid) -> Result<()> {
        // First verify the restaurant belongs to the owner
        self.verify_restaurant_ownership(restaurant_id, owner_id).await?;

        let result = sqlx::query(
            "DELETE FROM menu_items WHERE id = $1 AND restaurant_id = $2"
        )
        .bind(item_id)
        .bind(restaurant_id)
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Menu item not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_restaurant_orders(&self, restaurant_id: Uuid, owner_id: Uuid, status: Option<String>) -> Result<Vec<serde_json::Value>> {
        // First verify the restaurant belongs to the owner
        self.verify_restaurant_ownership(restaurant_id, owner_id).await?;

        let mut query = "SELECT * FROM orders WHERE restaurant_id = $1".to_string();
        let mut bind_count = 2;

        if status.is_some() {
            query.push_str(&format!(" AND status = ${}", bind_count));
            #[allow(unused_assignments)]
            {
                bind_count += 1;
            }
        }

        query.push_str(" ORDER BY created_at DESC");

        let mut query_builder = sqlx::query(&query);
        query_builder = query_builder.bind(restaurant_id);

        if let Some(ref status_val) = status {
            query_builder = query_builder.bind(status_val);
        }

        let rows = query_builder
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Convert rows to JSON for now
        let orders: Vec<serde_json::Value> = rows
            .into_iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<Uuid, _>("id"),
                    "customer_id": row.get::<Uuid, _>("customer_id"),
                    "restaurant_id": row.get::<Uuid, _>("restaurant_id"),
                    "status": row.get::<String, _>("status"),
                    "total_amount": row.get::<f64, _>("total_amount"),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
                    "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
                })
            })
            .collect();

        Ok(orders)
    }

    pub async fn update_restaurant_status(&self, restaurant_id: Uuid, owner_id: Uuid, is_accepting_orders: bool) -> Result<Restaurant> {
        let restaurant = sqlx::query_as::<_, Restaurant>(
            "UPDATE restaurants SET is_accepting_orders = $1, updated_at = $2 WHERE id = $3 AND owner_id = $4 AND is_active = true RETURNING *"
        )
        .bind(is_accepting_orders)
        .bind(Utc::now())
        .bind(restaurant_id)
        .bind(owner_id)
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Restaurant not found or not owned by user".to_string()))?;

        Ok(restaurant)
    }

    // Helper method to verify restaurant ownership
    async fn verify_restaurant_ownership(&self, restaurant_id: Uuid, owner_id: Uuid) -> Result<()> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM restaurants WHERE id = $1 AND owner_id = $2 AND is_active = true)"
        )
        .bind(restaurant_id)
        .bind(owner_id)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !exists {
            return Err(AppError::Forbidden("Restaurant not found or not owned by user".to_string()));
        }

        Ok(())
    }
}