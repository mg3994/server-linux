use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::models::User;
use axum::Extension;
use crate::error::Result;
use crate::restaurants::{
    models::*,
    service::RestaurantService,
};
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct RestaurantListQuery {
    pub city: Option<String>,
    pub cuisine_type: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RestaurantSearchQuery {
    pub q: String,
    pub city: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct OrderStatusQuery {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRestaurantStatusRequest {
    pub is_accepting_orders: bool,
}

// Restaurant CRUD Operations
pub async fn create_restaurant(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<CreateRestaurantRequest>,
) -> Result<Json<RestaurantResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let restaurant = restaurant_service
        .create_restaurant(user.id, request)
        .await?;
    
    Ok(Json(RestaurantResponse::from(restaurant)))
}

pub async fn get_restaurant(
    State(state): State<AppState>,
    Path(restaurant_id): Path<Uuid>,
) -> Result<Json<RestaurantResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let restaurant = restaurant_service
        .get_restaurant(restaurant_id)
        .await?;
    
    Ok(Json(RestaurantResponse::from(restaurant)))
}

pub async fn update_restaurant(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(restaurant_id): Path<Uuid>,
    Json(request): Json<UpdateRestaurantRequest>,
) -> Result<Json<RestaurantResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let restaurant = restaurant_service
        .update_restaurant(restaurant_id, user.id, request)
        .await?;
    
    Ok(Json(RestaurantResponse::from(restaurant)))
}

pub async fn delete_restaurant(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(restaurant_id): Path<Uuid>,
) -> Result<StatusCode> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    restaurant_service
        .delete_restaurant(restaurant_id, user.id)
        .await?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_restaurants(
    State(state): State<AppState>,
    Query(params): Query<RestaurantListQuery>,
) -> Result<Json<RestaurantListResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100); // Max 100 per page
    
    let response = restaurant_service
        .list_restaurants(params.city, params.cuisine_type, page, per_page)
        .await?;
    
    Ok(Json(response))
}

pub async fn search_restaurants(
    State(state): State<AppState>,
    Query(params): Query<RestaurantSearchQuery>,
) -> Result<Json<RestaurantListResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100); // Max 100 per page
    
    let response = restaurant_service
        .search_restaurants(params.q, params.city, page, per_page)
        .await?;
    
    Ok(Json(response))
}

// Menu Management
pub async fn create_menu_item(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(restaurant_id): Path<Uuid>,
    Json(request): Json<CreateMenuItemRequest>,
) -> Result<Json<MenuItemResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let menu_item = restaurant_service
        .create_menu_item(restaurant_id, user.id, request)
        .await?;
    
    Ok(Json(MenuItemResponse::from(menu_item)))
}

pub async fn get_menu(
    State(state): State<AppState>,
    Path(restaurant_id): Path<Uuid>,
) -> Result<Json<MenuResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let menu = restaurant_service
        .get_menu(restaurant_id)
        .await?;
    
    Ok(Json(menu))
}

pub async fn update_menu_item(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path((restaurant_id, item_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateMenuItemRequest>,
) -> Result<Json<MenuItemResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let menu_item = restaurant_service
        .update_menu_item(restaurant_id, item_id, user.id, request)
        .await?;
    
    Ok(Json(MenuItemResponse::from(menu_item)))
}

pub async fn delete_menu_item(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path((restaurant_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    restaurant_service
        .delete_menu_item(restaurant_id, item_id, user.id)
        .await?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Restaurant Operations
pub async fn get_restaurant_orders(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(restaurant_id): Path<Uuid>,
    Query(params): Query<OrderStatusQuery>,
) -> Result<Json<Vec<serde_json::Value>>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let orders = restaurant_service
        .get_restaurant_orders(restaurant_id, user.id, params.status)
        .await?;
    
    Ok(Json(orders))
}

pub async fn update_restaurant_status(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(restaurant_id): Path<Uuid>,
    Json(request): Json<UpdateRestaurantStatusRequest>,
) -> Result<Json<RestaurantResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let restaurant = restaurant_service
        .update_restaurant_status(restaurant_id, user.id, request.is_accepting_orders)
        .await?;
    
    Ok(Json(RestaurantResponse::from(restaurant)))
}

// India-specific restaurant endpoints
pub async fn get_restaurants_by_city(
    State(state): State<AppState>,
    Path(city): Path<String>,
    Query(params): Query<RestaurantListQuery>,
) -> Result<Json<RestaurantListResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    
    let response = restaurant_service
        .list_restaurants(Some(city), params.cuisine_type, page, per_page)
        .await?;
    
    Ok(Json(response))
}

pub async fn get_restaurants_by_cuisine(
    State(state): State<AppState>,
    Path(cuisine_type): Path<String>,
    Query(params): Query<RestaurantListQuery>,
) -> Result<Json<RestaurantListResponse>> {
    let restaurant_service = RestaurantService::new(state.database.clone());
    
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    
    let response = restaurant_service
        .list_restaurants(params.city, Some(cuisine_type), page, per_page)
        .await?;
    
    Ok(Json(response))
}