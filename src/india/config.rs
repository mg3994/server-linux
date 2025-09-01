use serde::{Deserialize, Serialize};

/// India-specific server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndiaConfig {
    pub default_currency: String,
    pub default_timezone: String,
    pub default_language: String,
    pub gst_enabled: bool,
    pub gst_number: Option<String>,
    pub fssai_license: Option<String>, // Food Safety and Standards Authority of India
    pub delivery_zones: Vec<String>,
    pub supported_payment_methods: Vec<String>,
    pub minimum_order_amount: f64,
    pub delivery_fee: f64,
    pub free_delivery_above: f64,
    pub peak_hour_surcharge: f64,
    pub weekend_surcharge: f64,
}

impl Default for IndiaConfig {
    fn default() -> Self {
        Self {
            default_currency: "INR".to_string(),
            default_timezone: "Asia/Kolkata".to_string(),
            default_language: "en-IN".to_string(),
            gst_enabled: true,
            gst_number: None,
            fssai_license: None,
            delivery_zones: vec![
                "Mumbai".to_string(),
                "Delhi".to_string(),
                "Bangalore".to_string(),
                "Chennai".to_string(),
                "Kolkata".to_string(),
                "Hyderabad".to_string(),
                "Pune".to_string(),
                "Ahmedabad".to_string(),
            ],
            supported_payment_methods: vec![
                "UPI".to_string(),
                "Credit Card".to_string(),
                "Debit Card".to_string(),
                "Net Banking".to_string(),
                "Paytm".to_string(),
                "PhonePe".to_string(),
                "Google Pay".to_string(),
                "Cash on Delivery".to_string(),
            ],
            minimum_order_amount: 99.0,  // ₹99
            delivery_fee: 29.0,          // ₹29
            free_delivery_above: 299.0,  // Free delivery above ₹299
            peak_hour_surcharge: 15.0,   // ₹15 during peak hours
            weekend_surcharge: 10.0,     // ₹10 on weekends
        }
    }
}

/// Indian Standard Time configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISTConfig {
    pub timezone: String,
    pub offset_hours: i8,
    pub offset_minutes: i8,
}

impl Default for ISTConfig {
    fn default() -> Self {
        Self {
            timezone: "Asia/Kolkata".to_string(),
            offset_hours: 5,
            offset_minutes: 30,
        }
    }
}

/// Business hours configuration for Indian market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub restaurant_open: String,    // "06:00"
    pub restaurant_close: String,   // "23:00"
    pub delivery_start: String,     // "07:00"
    pub delivery_end: String,       // "23:30"
    pub peak_hours: Vec<PeakHour>,
    pub weekend_hours: Option<WeekendHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakHour {
    pub start: String,
    pub end: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekendHours {
    pub saturday_open: String,
    pub saturday_close: String,
    pub sunday_open: String,
    pub sunday_close: String,
}

impl Default for BusinessHours {
    fn default() -> Self {
        Self {
            restaurant_open: "06:00".to_string(),
            restaurant_close: "23:00".to_string(),
            delivery_start: "07:00".to_string(),
            delivery_end: "23:30".to_string(),
            peak_hours: vec![
                PeakHour {
                    start: "12:00".to_string(),
                    end: "14:00".to_string(),
                    description: "Lunch Rush".to_string(),
                },
                PeakHour {
                    start: "19:00".to_string(),
                    end: "21:30".to_string(),
                    description: "Dinner Rush".to_string(),
                },
            ],
            weekend_hours: Some(WeekendHours {
                saturday_open: "08:00".to_string(),
                saturday_close: "23:30".to_string(),
                sunday_open: "08:00".to_string(),
                sunday_close: "22:00".to_string(),
            }),
        }
    }
}

/// Localization settings for Indian market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationConfig {
    pub supported_languages: Vec<IndianLanguage>,
    pub default_language: IndianLanguage,
    pub currency_format: CurrencyFormat,
    pub date_format: String,
    pub time_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndianLanguage {
    English,
    Hindi,
    Bengali,
    Telugu,
    Marathi,
    Tamil,
    Gujarati,
    Urdu,
    Kannada,
    Odia,
    Malayalam,
    Punjabi,
}

impl IndianLanguage {
    pub fn code(&self) -> &'static str {
        match self {
            IndianLanguage::English => "en-IN",
            IndianLanguage::Hindi => "hi-IN",
            IndianLanguage::Bengali => "bn-IN",
            IndianLanguage::Telugu => "te-IN",
            IndianLanguage::Marathi => "mr-IN",
            IndianLanguage::Tamil => "ta-IN",
            IndianLanguage::Gujarati => "gu-IN",
            IndianLanguage::Urdu => "ur-IN",
            IndianLanguage::Kannada => "kn-IN",
            IndianLanguage::Odia => "or-IN",
            IndianLanguage::Malayalam => "ml-IN",
            IndianLanguage::Punjabi => "pa-IN",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            IndianLanguage::English => "English",
            IndianLanguage::Hindi => "हिन्दी",
            IndianLanguage::Bengali => "বাংলা",
            IndianLanguage::Telugu => "తెలుగు",
            IndianLanguage::Marathi => "मराठी",
            IndianLanguage::Tamil => "தமிழ்",
            IndianLanguage::Gujarati => "ગુજરાતી",
            IndianLanguage::Urdu => "اردو",
            IndianLanguage::Kannada => "ಕನ್ನಡ",
            IndianLanguage::Odia => "ଓଡ଼ିଆ",
            IndianLanguage::Malayalam => "മലയാളം",
            IndianLanguage::Punjabi => "ਪੰਜਾਬੀ",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyFormat {
    pub symbol: String,
    pub position: CurrencyPosition,
    pub decimal_places: u8,
    pub thousands_separator: String,
    pub decimal_separator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrencyPosition {
    Before,
    After,
}

impl Default for CurrencyFormat {
    fn default() -> Self {
        Self {
            symbol: "₹".to_string(),
            position: CurrencyPosition::Before,
            decimal_places: 2,
            thousands_separator: ",".to_string(),
            decimal_separator: ".".to_string(),
        }
    }
}

impl Default for LocalizationConfig {
    fn default() -> Self {
        Self {
            supported_languages: vec![
                IndianLanguage::English,
                IndianLanguage::Hindi,
                IndianLanguage::Bengali,
                IndianLanguage::Tamil,
                IndianLanguage::Telugu,
                IndianLanguage::Marathi,
                IndianLanguage::Gujarati,
                IndianLanguage::Kannada,
            ],
            default_language: IndianLanguage::English,
            currency_format: CurrencyFormat::default(),
            date_format: "DD/MM/YYYY".to_string(),
            time_format: "HH:mm".to_string(),
        }
    }
}

/// Regulatory compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub fssai_required: bool,
    pub gst_registration_required: bool,
    pub data_localization_required: bool,
    pub customer_data_retention_days: u32,
    pub transaction_data_retention_days: u32,
    pub audit_log_retention_days: u32,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            fssai_required: true,
            gst_registration_required: true,
            data_localization_required: true,
            customer_data_retention_days: 365,      // 1 year
            transaction_data_retention_days: 2555,  // 7 years (as per RBI guidelines)
            audit_log_retention_days: 2555,         // 7 years
        }
    }
}