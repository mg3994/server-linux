use serde::{Deserialize, Serialize};


/// India-specific configuration and utilities
pub mod config;
pub mod payments;
pub mod handlers;

/// Indian states and their codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndianState {
    AndhraPradesh,
    ArunachalPradesh,
    Assam,
    Bihar,
    Chhattisgarh,
    Goa,
    Gujarat,
    Haryana,
    HimachalPradesh,
    Jharkhand,
    Karnataka,
    Kerala,
    MadhyaPradesh,
    Maharashtra,
    Manipur,
    Meghalaya,
    Mizoram,
    Nagaland,
    Odisha,
    Punjab,
    Rajasthan,
    Sikkim,
    TamilNadu,
    Telangana,
    Tripura,
    UttarPradesh,
    Uttarakhand,
    WestBengal,
    // Union Territories
    AndamanAndNicobarIslands,
    Chandigarh,
    DadraAndNagarHaveliAndDamanAndDiu,
    Delhi,
    Jammu,
    Kashmir,
    Ladakh,
    Lakshadweep,
    Puducherry,
}

impl IndianState {
    pub fn code(&self) -> &'static str {
        match self {
            IndianState::AndhraPradesh => "AP",
            IndianState::ArunachalPradesh => "AR",
            IndianState::Assam => "AS",
            IndianState::Bihar => "BR",
            IndianState::Chhattisgarh => "CG",
            IndianState::Goa => "GA",
            IndianState::Gujarat => "GJ",
            IndianState::Haryana => "HR",
            IndianState::HimachalPradesh => "HP",
            IndianState::Jharkhand => "JH",
            IndianState::Karnataka => "KA",
            IndianState::Kerala => "KL",
            IndianState::MadhyaPradesh => "MP",
            IndianState::Maharashtra => "MH",
            IndianState::Manipur => "MN",
            IndianState::Meghalaya => "ML",
            IndianState::Mizoram => "MZ",
            IndianState::Nagaland => "NL",
            IndianState::Odisha => "OR",
            IndianState::Punjab => "PB",
            IndianState::Rajasthan => "RJ",
            IndianState::Sikkim => "SK",
            IndianState::TamilNadu => "TN",
            IndianState::Telangana => "TS",
            IndianState::Tripura => "TR",
            IndianState::UttarPradesh => "UP",
            IndianState::Uttarakhand => "UK",
            IndianState::WestBengal => "WB",
            IndianState::AndamanAndNicobarIslands => "AN",
            IndianState::Chandigarh => "CH",
            IndianState::DadraAndNagarHaveliAndDamanAndDiu => "DN",
            IndianState::Delhi => "DL",
            IndianState::Jammu => "JK",
            IndianState::Kashmir => "JK",
            IndianState::Ladakh => "LA",
            IndianState::Lakshadweep => "LD",
            IndianState::Puducherry => "PY",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            IndianState::AndhraPradesh => "Andhra Pradesh",
            IndianState::ArunachalPradesh => "Arunachal Pradesh",
            IndianState::Assam => "Assam",
            IndianState::Bihar => "Bihar",
            IndianState::Chhattisgarh => "Chhattisgarh",
            IndianState::Goa => "Goa",
            IndianState::Gujarat => "Gujarat",
            IndianState::Haryana => "Haryana",
            IndianState::HimachalPradesh => "Himachal Pradesh",
            IndianState::Jharkhand => "Jharkhand",
            IndianState::Karnataka => "Karnataka",
            IndianState::Kerala => "Kerala",
            IndianState::MadhyaPradesh => "Madhya Pradesh",
            IndianState::Maharashtra => "Maharashtra",
            IndianState::Manipur => "Manipur",
            IndianState::Meghalaya => "Meghalaya",
            IndianState::Mizoram => "Mizoram",
            IndianState::Nagaland => "Nagaland",
            IndianState::Odisha => "Odisha",
            IndianState::Punjab => "Punjab",
            IndianState::Rajasthan => "Rajasthan",
            IndianState::Sikkim => "Sikkim",
            IndianState::TamilNadu => "Tamil Nadu",
            IndianState::Telangana => "Telangana",
            IndianState::Tripura => "Tripura",
            IndianState::UttarPradesh => "Uttar Pradesh",
            IndianState::Uttarakhand => "Uttarakhand",
            IndianState::WestBengal => "West Bengal",
            IndianState::AndamanAndNicobarIslands => "Andaman and Nicobar Islands",
            IndianState::Chandigarh => "Chandigarh",
            IndianState::DadraAndNagarHaveliAndDamanAndDiu => "Dadra and Nagar Haveli and Daman and Diu",
            IndianState::Delhi => "Delhi",
            IndianState::Jammu => "Jammu",
            IndianState::Kashmir => "Kashmir",
            IndianState::Ladakh => "Ladakh",
            IndianState::Lakshadweep => "Lakshadweep",
            IndianState::Puducherry => "Puducherry",
        }
    }
}

/// Major Indian cities with coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianCity {
    pub name: String,
    pub state: IndianState,
    pub latitude: f64,
    pub longitude: f64,
    pub is_metro: bool,
    pub population: Option<u32>,
}

impl IndianCity {
    pub fn get_major_cities() -> Vec<IndianCity> {
        vec![
            IndianCity {
                name: "Mumbai".to_string(),
                state: IndianState::Maharashtra,
                latitude: 19.0760,
                longitude: 72.8777,
                is_metro: true,
                population: Some(12442373),
            },
            IndianCity {
                name: "Delhi".to_string(),
                state: IndianState::Delhi,
                latitude: 28.6139,
                longitude: 77.2090,
                is_metro: true,
                population: Some(11007835),
            },
            IndianCity {
                name: "Bangalore".to_string(),
                state: IndianState::Karnataka,
                latitude: 12.9716,
                longitude: 77.5946,
                is_metro: true,
                population: Some(8443675),
            },
            IndianCity {
                name: "Hyderabad".to_string(),
                state: IndianState::Telangana,
                latitude: 17.3850,
                longitude: 78.4867,
                is_metro: true,
                population: Some(6809970),
            },
            IndianCity {
                name: "Ahmedabad".to_string(),
                state: IndianState::Gujarat,
                latitude: 23.0225,
                longitude: 72.5714,
                is_metro: false,
                population: Some(5570585),
            },
            IndianCity {
                name: "Chennai".to_string(),
                state: IndianState::TamilNadu,
                latitude: 13.0827,
                longitude: 80.2707,
                is_metro: true,
                population: Some(4681087),
            },
            IndianCity {
                name: "Kolkata".to_string(),
                state: IndianState::WestBengal,
                latitude: 22.5726,
                longitude: 88.3639,
                is_metro: true,
                population: Some(4496694),
            },
            IndianCity {
                name: "Pune".to_string(),
                state: IndianState::Maharashtra,
                latitude: 18.5204,
                longitude: 73.8567,
                is_metro: false,
                population: Some(3124458),
            },
        ]
    }
}

/// Indian cuisine types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndianCuisine {
    NorthIndian,
    SouthIndian,
    Punjabi,
    Gujarati,
    Rajasthani,
    Bengali,
    Maharashtrian,
    Tamil,
    Kerala,
    Hyderabadi,
    Mughlai,
    Kashmiri,
    Goan,
    Assamese,
    Bihari,
    Odia,
    Street,
    Chaat,
    Dosa,
    Biryani,
    Chinese,
    Continental,
    Italian,
    Mexican,
    Thai,
    Japanese,
    Lebanese,
}

impl IndianCuisine {
    pub fn name(&self) -> &'static str {
        match self {
            IndianCuisine::NorthIndian => "North Indian",
            IndianCuisine::SouthIndian => "South Indian",
            IndianCuisine::Punjabi => "Punjabi",
            IndianCuisine::Gujarati => "Gujarati",
            IndianCuisine::Rajasthani => "Rajasthani",
            IndianCuisine::Bengali => "Bengali",
            IndianCuisine::Maharashtrian => "Maharashtrian",
            IndianCuisine::Tamil => "Tamil",
            IndianCuisine::Kerala => "Kerala",
            IndianCuisine::Hyderabadi => "Hyderabadi",
            IndianCuisine::Mughlai => "Mughlai",
            IndianCuisine::Kashmiri => "Kashmiri",
            IndianCuisine::Goan => "Goan",
            IndianCuisine::Assamese => "Assamese",
            IndianCuisine::Bihari => "Bihari",
            IndianCuisine::Odia => "Odia",
            IndianCuisine::Street => "Street Food",
            IndianCuisine::Chaat => "Chaat",
            IndianCuisine::Dosa => "Dosa",
            IndianCuisine::Biryani => "Biryani",
            IndianCuisine::Chinese => "Chinese",
            IndianCuisine::Continental => "Continental",
            IndianCuisine::Italian => "Italian",
            IndianCuisine::Mexican => "Mexican",
            IndianCuisine::Thai => "Thai",
            IndianCuisine::Japanese => "Japanese",
            IndianCuisine::Lebanese => "Lebanese",
        }
    }
}

/// Indian food delivery time zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTimeZone {
    pub zone_name: String,
    pub cities: Vec<String>,
    pub standard_delivery_time: u32, // minutes
    pub peak_hour_multiplier: f32,
    pub weekend_multiplier: f32,
}

impl DeliveryTimeZone {
    pub fn get_zones() -> Vec<DeliveryTimeZone> {
        vec![
            DeliveryTimeZone {
                zone_name: "Metro Cities".to_string(),
                cities: vec!["Mumbai".to_string(), "Delhi".to_string(), "Bangalore".to_string(), "Chennai".to_string(), "Kolkata".to_string(), "Hyderabad".to_string()],
                standard_delivery_time: 30,
                peak_hour_multiplier: 1.5,
                weekend_multiplier: 1.2,
            },
            DeliveryTimeZone {
                zone_name: "Tier 1 Cities".to_string(),
                cities: vec!["Pune".to_string(), "Ahmedabad".to_string(), "Jaipur".to_string(), "Lucknow".to_string(), "Kanpur".to_string(), "Nagpur".to_string()],
                standard_delivery_time: 35,
                peak_hour_multiplier: 1.4,
                weekend_multiplier: 1.3,
            },
            DeliveryTimeZone {
                zone_name: "Tier 2 Cities".to_string(),
                cities: vec!["Indore".to_string(), "Bhopal".to_string(), "Coimbatore".to_string(), "Kochi".to_string(), "Chandigarh".to_string()],
                standard_delivery_time: 40,
                peak_hour_multiplier: 1.3,
                weekend_multiplier: 1.4,
            },
        ]
    }
}

/// GST (Goods and Services Tax) rates for food items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GSTRate {
    pub category: String,
    pub rate: f64, // percentage
    pub description: String,
}

impl GSTRate {
    pub fn get_food_gst_rates() -> Vec<GSTRate> {
        vec![
            GSTRate {
                category: "Restaurant Service".to_string(),
                rate: 5.0,
                description: "GST on restaurant services (non-AC restaurants)".to_string(),
            },
            GSTRate {
                category: "AC Restaurant Service".to_string(),
                rate: 18.0,
                description: "GST on AC restaurant services".to_string(),
            },
            GSTRate {
                category: "Delivery Charges".to_string(),
                rate: 18.0,
                description: "GST on delivery and logistics services".to_string(),
            },
            GSTRate {
                category: "Packaged Food".to_string(),
                rate: 12.0,
                description: "GST on packaged food items".to_string(),
            },
        ]
    }

    pub fn calculate_gst(&self, amount: f64) -> f64 {
        amount * (self.rate / 100.0)
    }
}

/// Indian festival and holiday calendar affecting delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianFestival {
    pub name: String,
    pub date: String, // Format: "MM-DD" for recurring festivals
    pub is_national_holiday: bool,
    pub affects_delivery: bool,
    pub delivery_impact: String,
}

impl IndianFestival {
    pub fn get_major_festivals() -> Vec<IndianFestival> {
        vec![
            IndianFestival {
                name: "Diwali".to_string(),
                date: "Variable".to_string(),
                is_national_holiday: true,
                affects_delivery: true,
                delivery_impact: "High demand, extended delivery times".to_string(),
            },
            IndianFestival {
                name: "Holi".to_string(),
                date: "Variable".to_string(),
                is_national_holiday: true,
                affects_delivery: true,
                delivery_impact: "Limited service in some areas".to_string(),
            },
            IndianFestival {
                name: "Eid".to_string(),
                date: "Variable".to_string(),
                is_national_holiday: true,
                affects_delivery: true,
                delivery_impact: "High demand for special foods".to_string(),
            },
            IndianFestival {
                name: "Independence Day".to_string(),
                date: "08-15".to_string(),
                is_national_holiday: true,
                affects_delivery: false,
                delivery_impact: "Normal operations".to_string(),
            },
            IndianFestival {
                name: "Republic Day".to_string(),
                date: "01-26".to_string(),
                is_national_holiday: true,
                affects_delivery: false,
                delivery_impact: "Normal operations".to_string(),
            },
        ]
    }
}