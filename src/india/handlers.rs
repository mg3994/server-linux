use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::india::{IndianCity, IndianState, IndianCuisine, GSTRate, DeliveryTimeZone};
use crate::india::payments::{UPIApp, IndianBank, PaymentGatewayFees};
use crate::india::config::{IndiaConfig, LocalizationConfig};

/// Get supported Indian cities
pub async fn get_supported_cities() -> Result<Json<Vec<IndianCity>>> {
    Ok(Json(IndianCity::get_major_cities()))
}

/// Get Indian states
pub async fn get_indian_states() -> Result<Json<Vec<StateInfo>>> {
    let states: Vec<StateInfo> = vec![
        IndianState::Maharashtra,
        IndianState::Karnataka,
        IndianState::Delhi,
        IndianState::TamilNadu,
        IndianState::WestBengal,
        IndianState::Telangana,
        IndianState::Gujarat,
        IndianState::Rajasthan,
        IndianState::UttarPradesh,
        IndianState::Punjab,
    ]
    .into_iter()
    .map(|state| StateInfo {
        code: state.code().to_string(),
        name: state.name().to_string(),
    })
    .collect();

    Ok(Json(states))
}

/// Get Indian cuisine types
pub async fn get_cuisine_types() -> Result<Json<Vec<CuisineInfo>>> {
    let cuisines: Vec<CuisineInfo> = vec![
        IndianCuisine::NorthIndian,
        IndianCuisine::SouthIndian,
        IndianCuisine::Punjabi,
        IndianCuisine::Gujarati,
        IndianCuisine::Bengali,
        IndianCuisine::Maharashtrian,
        IndianCuisine::Tamil,
        IndianCuisine::Hyderabadi,
        IndianCuisine::Street,
        IndianCuisine::Biryani,
        IndianCuisine::Chinese,
        IndianCuisine::Continental,
    ]
    .into_iter()
    .map(|cuisine| CuisineInfo {
        name: cuisine.name().to_string(),
    })
    .collect();

    Ok(Json(cuisines))
}

/// Get GST rates for food items
pub async fn get_gst_rates() -> Result<Json<Vec<GSTRate>>> {
    Ok(Json(GSTRate::get_food_gst_rates()))
}

/// Calculate GST for an amount
pub async fn calculate_gst(
    Query(params): Query<GSTCalculationRequest>,
) -> Result<Json<GSTCalculationResponse>> {
    let gst_rates = GSTRate::get_food_gst_rates();
    let rate = gst_rates
        .iter()
        .find(|r| r.category == params.category)
        .unwrap_or(&gst_rates[0]); // Default to first rate if not found

    let gst_amount = rate.calculate_gst(params.amount);
    let total_amount = params.amount + gst_amount;

    Ok(Json(GSTCalculationResponse {
        base_amount: params.amount,
        gst_rate: rate.rate,
        gst_amount,
        total_amount,
        category: rate.category.clone(),
    }))
}

/// Get supported UPI apps
pub async fn get_upi_apps() -> Result<Json<Vec<UPIAppInfo>>> {
    let apps: Vec<UPIAppInfo> = vec![
        UPIApp::GooglePay,
        UPIApp::PhonePe,
        UPIApp::Paytm,
        UPIApp::AmazonPay,
        UPIApp::BHIM,
        UPIApp::MobiKwik,
    ]
    .into_iter()
    .map(|app| UPIAppInfo {
        name: app.name().to_string(),
        package_name: app.package_name().to_string(),
    })
    .collect();

    Ok(Json(apps))
}

/// Get supported Indian banks
pub async fn get_indian_banks() -> Result<Json<Vec<BankInfo>>> {
    let banks: Vec<BankInfo> = vec![
        IndianBank::SBI,
        IndianBank::HDFC,
        IndianBank::ICICI,
        IndianBank::Axis,
        IndianBank::Kotak,
        IndianBank::IndusInd,
        IndianBank::YesBank,
        IndianBank::PNB,
    ]
    .into_iter()
    .map(|bank| BankInfo {
        name: bank.name().to_string(),
        code: bank.code().to_string(),
    })
    .collect();

    Ok(Json(banks))
}

/// Get payment gateway fees
pub async fn get_payment_fees() -> Result<Json<Vec<PaymentGatewayFees>>> {
    Ok(Json(PaymentGatewayFees::get_standard_fees()))
}

/// Get delivery time zones
pub async fn get_delivery_zones() -> Result<Json<Vec<DeliveryTimeZone>>> {
    Ok(Json(DeliveryTimeZone::get_zones()))
}

/// Get India-specific configuration
pub async fn get_india_config() -> Result<Json<IndiaConfig>> {
    Ok(Json(IndiaConfig::default()))
}

/// Get localization configuration
pub async fn get_localization_config() -> Result<Json<LocalizationConfig>> {
    Ok(Json(LocalizationConfig::default()))
}

/// Calculate delivery time based on city and current conditions
pub async fn calculate_delivery_time(
    Query(params): Query<DeliveryTimeRequest>,
) -> Result<Json<DeliveryTimeResponse>> {
    let zones = DeliveryTimeZone::get_zones();
    let zone = zones
        .iter()
        .find(|z| z.cities.contains(&params.city))
        .unwrap_or(&zones[2]); // Default to Tier 2 if city not found

    let mut delivery_time = zone.standard_delivery_time as f32;

    // Apply multipliers based on conditions
    if params.is_peak_hour.unwrap_or(false) {
        delivery_time *= zone.peak_hour_multiplier;
    }

    if params.is_weekend.unwrap_or(false) {
        delivery_time *= zone.weekend_multiplier;
    }

    // Add weather delay if applicable
    if let Some(weather_delay) = params.weather_delay_minutes {
        delivery_time += weather_delay as f32;
    }

    Ok(Json(DeliveryTimeResponse {
        city: params.city,
        estimated_delivery_minutes: delivery_time.round() as u32,
        zone_name: zone.zone_name.clone(),
        factors_applied: vec![
            if params.is_peak_hour.unwrap_or(false) {
                Some("Peak Hour Multiplier".to_string())
            } else {
                None
            },
            if params.is_weekend.unwrap_or(false) {
                Some("Weekend Multiplier".to_string())
            } else {
                None
            },
            if params.weather_delay_minutes.is_some() {
                Some("Weather Delay".to_string())
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect(),
    }))
}

// Request/Response models
#[derive(Debug, Serialize)]
pub struct StateInfo {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CuisineInfo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GSTCalculationRequest {
    pub amount: f64,
    pub category: String,
}

#[derive(Debug, Serialize)]
pub struct GSTCalculationResponse {
    pub base_amount: f64,
    pub gst_rate: f64,
    pub gst_amount: f64,
    pub total_amount: f64,
    pub category: String,
}

#[derive(Debug, Serialize)]
pub struct UPIAppInfo {
    pub name: String,
    pub package_name: String,
}

#[derive(Debug, Serialize)]
pub struct BankInfo {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryTimeRequest {
    pub city: String,
    pub is_peak_hour: Option<bool>,
    pub is_weekend: Option<bool>,
    pub weather_delay_minutes: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryTimeResponse {
    pub city: String,
    pub estimated_delivery_minutes: u32,
    pub zone_name: String,
    pub factors_applied: Vec<String>,
}