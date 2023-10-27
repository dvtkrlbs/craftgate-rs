use serde::{Deserialize, Serialize};

pub mod onboarding;
pub mod payment;

/// Status that can be used when communicating with the Craftgate API:
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// Active, represents records that available to be used
    Active,
    /// Passive, represents records that not available to be used
    Passive,
}

/// Currencies that can be used when communicating with the Craftgate API:
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    /// Turkish Lira
    Try,
    /// U.S. Dollar
    Usd,
    /// Euro
    Eur,
    /// British Pound
    Gbp,
    /// Chinese Yuan
    Cny,
    /// Argentina Peso
    Ars,
    /// Brazilian Real
    Brl,
    /// United Arab Emirates Dirham
    Aed,
    /// Iraqi Dinar
    Iqd,
    /// Azerbaijani Manat
    Azn,
    /// Kazakhstani Tenge
    Kzt,
}

/// Types expressing the configuration of the use of card:
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    /// Credit card
    CreditCard,
    /// Debit card
    DebitCard,
    /// Prepaid card
    PrepaidCard,
}
