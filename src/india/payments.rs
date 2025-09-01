use crate::payments::models::PaymentMethod;
use serde::{Deserialize, Serialize};

/// UPI (Unified Payments Interface) specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPIConfig {
    pub merchant_id: String,
    pub merchant_name: String,
    pub transaction_note: String,
    pub currency: String,
}

impl Default for UPIConfig {
    fn default() -> Self {
        Self {
            merchant_id: "MERCHANT001".to_string(),
            merchant_name: "FoodDelivery".to_string(),
            transaction_note: "Food Order Payment".to_string(),
            currency: "INR".to_string(),
        }
    }
}

impl UPIConfig {
    /// Generate UPI payment URL
    pub fn generate_payment_url(&self, amount: f64, transaction_id: &str) -> String {
        format!(
            "upi://pay?pa={}&pn={}&am={:.2}&cu={}&tn={}&tr={}",
            self.merchant_id,
            self.merchant_name,
            amount,
            self.currency,
            self.transaction_note,
            transaction_id
        )
    }
}

/// Popular UPI apps in India
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UPIApp {
    GooglePay,
    PhonePe,
    Paytm,
    AmazonPay,
    BHIM,
    MobiKwik,
    FreeCharge,
    PayZapp,
    IMobile,
    YesPayNext,
}

impl UPIApp {
    pub fn name(&self) -> &'static str {
        match self {
            UPIApp::GooglePay => "Google Pay",
            UPIApp::PhonePe => "PhonePe",
            UPIApp::Paytm => "Paytm",
            UPIApp::AmazonPay => "Amazon Pay",
            UPIApp::BHIM => "BHIM",
            UPIApp::MobiKwik => "MobiKwik",
            UPIApp::FreeCharge => "FreeCharge",
            UPIApp::PayZapp => "PayZapp",
            UPIApp::IMobile => "iMobile Pay",
            UPIApp::YesPayNext => "Yes Pay Next",
        }
    }

    pub fn package_name(&self) -> &'static str {
        match self {
            UPIApp::GooglePay => "com.google.android.apps.nfc.payment",
            UPIApp::PhonePe => "com.phonepe.app",
            UPIApp::Paytm => "net.one97.paytm",
            UPIApp::AmazonPay => "in.amazon.mShop.android.shopping",
            UPIApp::BHIM => "in.org.npci.upiapp",
            UPIApp::MobiKwik => "com.mobikwik_new",
            UPIApp::FreeCharge => "com.freecharge.android",
            UPIApp::PayZapp => "com.hdfcbank.payzapp",
            UPIApp::IMobile => "com.csam.icici.bank.imobile",
            UPIApp::YesPayNext => "com.yesbank.yespay",
        }
    }
}

/// Indian banking networks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankingNetwork {
    IMPS, // Immediate Payment Service
    NEFT, // National Electronic Funds Transfer
    RTGS, // Real Time Gross Settlement
    UPI,  // Unified Payments Interface
}

impl BankingNetwork {
    pub fn name(&self) -> &'static str {
        match self {
            BankingNetwork::IMPS => "IMPS",
            BankingNetwork::NEFT => "NEFT",
            BankingNetwork::RTGS => "RTGS",
            BankingNetwork::UPI => "UPI",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BankingNetwork::IMPS => "Immediate Payment Service - 24x7 instant money transfer",
            BankingNetwork::NEFT => "National Electronic Funds Transfer - Batch processing",
            BankingNetwork::RTGS => "Real Time Gross Settlement - High value transactions",
            BankingNetwork::UPI => "Unified Payments Interface - Instant mobile payments",
        }
    }

    pub fn min_amount(&self) -> f64 {
        match self {
            BankingNetwork::IMPS => 1.0,
            BankingNetwork::NEFT => 1.0,
            BankingNetwork::RTGS => 200000.0, // 2 lakhs minimum
            BankingNetwork::UPI => 1.0,
        }
    }

    pub fn max_amount(&self) -> f64 {
        match self {
            BankingNetwork::IMPS => 500000.0,   // 5 lakhs
            BankingNetwork::NEFT => 1000000.0,  // 10 lakhs
            BankingNetwork::RTGS => 10000000.0, // 1 crore
            BankingNetwork::UPI => 100000.0,    // 1 lakh
        }
    }
}

/// Payment gateway fees structure for India
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayFees {
    pub payment_method: PaymentMethod,
    pub percentage_fee: f64,
    pub fixed_fee: f64, // in INR
    pub gst_applicable: bool,
}

impl PaymentGatewayFees {
    pub fn get_standard_fees() -> Vec<PaymentGatewayFees> {
        vec![
            PaymentGatewayFees {
                payment_method: PaymentMethod::UPI,
                percentage_fee: 0.0, // UPI is typically free for merchants
                fixed_fee: 0.0,
                gst_applicable: false,
            },
            PaymentGatewayFees {
                payment_method: PaymentMethod::CreditCard,
                percentage_fee: 2.0,
                fixed_fee: 3.0,
                gst_applicable: true,
            },
            PaymentGatewayFees {
                payment_method: PaymentMethod::DebitCard,
                percentage_fee: 1.0,
                fixed_fee: 2.0,
                gst_applicable: true,
            },
            PaymentGatewayFees {
                payment_method: PaymentMethod::NetBanking,
                percentage_fee: 1.5,
                fixed_fee: 5.0,
                gst_applicable: true,
            },
            PaymentGatewayFees {
                payment_method: PaymentMethod::Paytm,
                percentage_fee: 1.0,
                fixed_fee: 1.0,
                gst_applicable: true,
            },
            PaymentGatewayFees {
                payment_method: PaymentMethod::PhonePe,
                percentage_fee: 1.0,
                fixed_fee: 1.0,
                gst_applicable: true,
            },
        ]
    }

    pub fn calculate_fee(&self, amount: f64) -> f64 {
        let base_fee = (amount * self.percentage_fee / 100.0) + self.fixed_fee;
        if self.gst_applicable {
            base_fee * 1.18 // Adding 18% GST
        } else {
            base_fee
        }
    }
}

/// RBI (Reserve Bank of India) compliance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBICompliance {
    pub two_factor_auth_required: bool,
    pub transaction_limit_per_day: f64,
    pub kyc_required_above: f64,
    pub additional_auth_above: f64,
}

impl Default for RBICompliance {
    fn default() -> Self {
        Self {
            two_factor_auth_required: true,
            transaction_limit_per_day: 100000.0, // 1 lakh per day
            kyc_required_above: 50000.0,         // 50k
            additional_auth_above: 10000.0,      // 10k
        }
    }
}

/// Popular Indian banks for net banking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndianBank {
    SBI,          // State Bank of India
    HDFC,         // HDFC Bank
    ICICI,        // ICICI Bank
    Axis,         // Axis Bank
    Kotak,        // Kotak Mahindra Bank
    IndusInd,     // IndusInd Bank
    YesBank,      // Yes Bank
    PNB,          // Punjab National Bank
    BankOfBaroda, // Bank of Baroda
    Canara,       // Canara Bank
    UnionBank,    // Union Bank of India
    IDBI,         // IDBI Bank
}

impl IndianBank {
    pub fn name(&self) -> &'static str {
        match self {
            IndianBank::SBI => "State Bank of India",
            IndianBank::HDFC => "HDFC Bank",
            IndianBank::ICICI => "ICICI Bank",
            IndianBank::Axis => "Axis Bank",
            IndianBank::Kotak => "Kotak Mahindra Bank",
            IndianBank::IndusInd => "IndusInd Bank",
            IndianBank::YesBank => "Yes Bank",
            IndianBank::PNB => "Punjab National Bank",
            IndianBank::BankOfBaroda => "Bank of Baroda",
            IndianBank::Canara => "Canara Bank",
            IndianBank::UnionBank => "Union Bank of India",
            IndianBank::IDBI => "IDBI Bank",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            IndianBank::SBI => "SBIN",
            IndianBank::HDFC => "HDFC",
            IndianBank::ICICI => "ICIC",
            IndianBank::Axis => "UTIB",
            IndianBank::Kotak => "KKBK",
            IndianBank::IndusInd => "INDB",
            IndianBank::YesBank => "YESB",
            IndianBank::PNB => "PUNB",
            IndianBank::BankOfBaroda => "BARB",
            IndianBank::Canara => "CNRB",
            IndianBank::UnionBank => "UBIN",
            IndianBank::IDBI => "IBKL",
        }
    }
}
